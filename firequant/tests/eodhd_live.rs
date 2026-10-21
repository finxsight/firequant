//! Live smoke tests against EODHD (requires `EODHD_API_KEY`).
//!
//! Run: `EODHD_API_KEY=... cargo test -p firequant --test eodhd_live -- --nocapture`

#![allow(clippy::unwrap_used, clippy::expect_used)]

use chrono::{NaiveDate, TimeZone, Utc};
use firequant::backtest::{backtest_simple_bool_signal, PerformanceMetrics};
use firequant::core::indicators::incremental::rsi::RSI;
use firequant::core::traits::Next;
use polars::prelude::*;
use serde::Deserialize;
use std::env;

const EOD_URL: &str = "https://eodhd.com/api/eod";

#[derive(Debug, Deserialize)]
struct EodBar {
    date: String,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    volume: f64,
}

fn api_token() -> Option<String> {
    env::var("EODHD_API_KEY")
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

fn fetch_eod(symbol: &str, from: &str) -> Vec<EodBar> {
    let token = api_token().expect("EODHD_API_KEY must be set for live tests");
    let url = format!(
        "{}/{}?api_token={}&fmt=json&period=d&from={}",
        EOD_URL,
        symbol,
        token,
        from
    );
    let client = reqwest::blocking::Client::builder()
        .user_agent("firequant/eodhd-live-test")
        .timeout(std::time::Duration::from_secs(90))
        .build()
        .expect("http client");
    let mut bars: Vec<EodBar> = client
        .get(&url)
        .send()
        .expect("eodhd request")
        .error_for_status()
        .expect("eodhd http status")
        .json()
        .expect("eodhd json");
    bars.sort_by(|a, b| a.date.cmp(&b.date));
    bars
}

fn date_to_ts(date: &str) -> i64 {
    let d = NaiveDate::parse_from_str(date, "%Y-%m-%d").expect("eod date");
    Utc.from_utc_datetime(&d.and_hms_opt(0, 0, 0).expect("midnight"))
        .timestamp()
}

fn rsi_series(closes: &[f64], period: usize) -> Vec<f64> {
    let mut rsi = RSI::new(period);
    closes.iter().map(|c| rsi.next(*c)).collect()
}

#[test]
fn eodhd_fetches_us_equity_bars() {
    if api_token().is_none() {
        eprintln!("skip eodhd_fetches_us_equity_bars: EODHD_API_KEY not set");
        return;
    }
    for symbol in ["AAPL.US", "MSFT.US", "SPY.US"] {
        let bars = fetch_eod(symbol, "2025-01-01");
        assert!(
            bars.len() >= 50,
            "{symbol}: expected >=50 daily bars, got {}",
            bars.len()
        );
        let last = bars.last().expect("last bar");
        assert!(last.close > 1.0, "{symbol}: close {}", last.close);
        assert!(last.high >= last.low);
        println!(
            "{symbol}: {} bars, last {} close={:.2}",
            bars.len(),
            last.date,
            last.close
        );
    }
}

#[test]
fn eodhd_rsi_and_backtest_on_aapl() {
    if api_token().is_none() {
        eprintln!("skip eodhd_rsi_and_backtest_on_aapl: EODHD_API_KEY not set");
        return;
    }
    let bars = fetch_eod("AAPL.US", "2024-01-01");
    assert!(bars.len() >= 100, "need history for RSI(14)");

    let closes: Vec<f64> = bars.iter().map(|b| b.close).collect();
    let rsis = rsi_series(&closes, 14);
    let last_rsi = *rsis.last().expect("rsi");
    assert!(
        last_rsi.is_finite() && last_rsi >= 0.0 && last_rsi <= 100.0,
        "rsi {}",
        last_rsi
    );
    println!("AAPL last RSI(14) = {:.2}", last_rsi);

    let timestamps: Vec<i64> = bars.iter().map(|b| date_to_ts(&b.date)).collect();
    let signals: Vec<f64> = rsis
        .iter()
        .map(|r| {
            if *r < 30.0 {
                1.0
            } else if *r > 70.0 {
                0.0
            } else {
                f64::NAN
            }
        })
        .map(|s| if s.is_nan() { 0.0 } else { s })
        .collect();

    let df = DataFrame::new(vec![
        Column::new("timestamp".into(), timestamps),
        Column::new("close".into(), closes),
        Column::new("signal".into(), signals),
    ])
    .expect("dataframe");

    let result = backtest_simple_bool_signal(df, "signal").expect("backtest");
    let metrics = PerformanceMetrics::from_result(&result);
    println!(
        "AAPL RSI mean-reversion smoke: trades={} final_equity={:.2} sharpe={:.3}",
        metrics.num_trades, metrics.final_equity, metrics.sharpe_ratio
    );
    assert!(metrics.final_equity > 0.0);
    assert!(result.equity_curve.height() == bars.len());
}
