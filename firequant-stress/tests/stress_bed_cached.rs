//! Stress bed on cached EODHD data (requires prior `eodhd_stress fetch`).

#![allow(clippy::unwrap_used)]

use firequant_stress::{TICKERS, cache_dir, load_series, run_stress_bed};

#[test]
fn stress_bed_five_functions_on_cached_universe() {
    let dir = cache_dir();
    let mut loaded = Vec::new();
    for symbol in TICKERS {
        if let Some(s) = load_series(&dir, symbol) {
            loaded.push(s);
        }
    }
    if loaded.len() < TICKERS.len() {
        eprintln!(
            "skip stress_bed: only {}/{} symbols cached — run: cargo run -p firequant-stress --bin eodhd_stress -- fetch",
            loaded.len(),
            TICKERS.len()
        );
        return;
    }
    let report = run_stress_bed(&loaded);
    assert!(
        report.failures.is_empty(),
        "failures: {:?}",
        report.failures
    );
    assert_eq!(report.results.len(), TICKERS.len());
    assert!(report.total_bars > 100_000, "bars {}", report.total_bars);
    println!(
        "stress ok: symbols={} bars={} ms={}",
        report.results.len(),
        report.total_bars,
        report.elapsed_ms
    );
}
