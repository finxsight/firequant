//! CLI: fetch full EODHD history for 100 tickers, cache, stress-test 5 quant paths.

use firequant_stress::{
    TICKERS, cache_dir, entry_from_series, fetch_full_history, load_manifest, load_series,
    run_stress_bed, save_series, write_manifest,
};
use firequant_stress::cache::bars_from_eodhd;
use std::thread;
use std::time::Duration;

fn main() {
    let cmd = std::env::args().nth(1).unwrap_or_else(|| "all".to_string());
    match cmd.as_str() {
        "fetch" => fetch_all(),
        "stress" => stress_all(),
        "all" => {
            fetch_all();
            stress_all();
        }
        other => {
            eprintln!("usage: eodhd_stress [fetch|stress|all]");
            eprintln!("unknown command: {other}");
            std::process::exit(1);
        }
    }
}

fn fetch_all() {
    let dir = cache_dir();
    println!("cache dir: {}", dir.display());
    let mut entries = Vec::new();
    let mut fetched = 0usize;
    let mut skipped = 0usize;

    for (i, symbol) in TICKERS.iter().enumerate() {
        if let Some(existing) = load_series(&dir, symbol) {
            if existing.bars.len() > 100 {
                entries.push(entry_from_series(&existing));
                skipped += 1;
                println!("[{}/{}] skip {} ({} bars cached)", i + 1, TICKERS.len(), symbol, existing.bars.len());
                continue;
            }
        }
        print!("[{}/{}] fetch {} ... ", i + 1, TICKERS.len(), symbol);
        match fetch_full_history(symbol) {
            Ok(bars) => {
                let series = bars_from_eodhd(symbol, bars);
                if let Err(e) = save_series(&dir, &series) {
                    eprintln!("save failed: {e}");
                    continue;
                }
                let ent = entry_from_series(&series);
                println!(
                    "{} bars ({} .. {})",
                    ent.bars,
                    ent.first_date.as_deref().unwrap_or("?"),
                    ent.last_date.as_deref().unwrap_or("?")
                );
                entries.push(ent);
                fetched += 1;
            }
            Err(e) => eprintln!("FAILED: {e}"),
        }
        thread::sleep(Duration::from_millis(300));
    }

    let entries: Vec<_> = TICKERS
        .iter()
        .filter_map(|s| load_series(&dir, s).map(|ser| entry_from_series(&ser)))
        .collect();
    if let Err(e) = write_manifest(&dir, &entries) {
        eprintln!("manifest write failed: {e}");
    }
    println!("done: fetched={} skipped={} manifest_symbols={}", fetched, skipped, entries.len());
}

fn stress_all() {
    let dir = cache_dir();
    if let Some(m) = load_manifest(&dir) {
        println!(
            "manifest: {} symbols, updated_unix={}",
            m.symbols.len(),
            m.updated_unix
        );
    }

    let mut series_list = Vec::new();
    for symbol in TICKERS {
        match load_series(&dir, symbol) {
            Some(s) => series_list.push(s),
            None => eprintln!("missing cache for {symbol} — run fetch first"),
        }
    }

    if series_list.is_empty() {
        eprintln!("no cached data");
        std::process::exit(1);
    }

    println!("stress bed: {} tickers loaded", series_list.len());
    let report = run_stress_bed(&series_list);
    println!(
        "stress: ok={} failures={} total_bars={} elapsed_ms={}",
        report.results.len(),
        report.failures.len(),
        report.total_bars,
        report.elapsed_ms
    );

    for f in &report.failures {
        eprintln!("  fail: {f}");
    }

    for r in report.results.iter().take(5) {
        println!(
            "  sample {} bars={} rsi={:.2} ema={:.2} trades={}",
            r.symbol,
            r.bars,
            r.last_rsi,
            r.last_ema,
            r.backtest_trades
        );
    }

    if !report.failures.is_empty() || report.results.len() < TICKERS.len() / 2 {
        std::process::exit(1);
    }
}
