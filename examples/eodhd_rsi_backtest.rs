//! EODHD → RSI → simple backtest (requires `EODHD_API_KEY`).
//!
//! `cargo run -p firequant --example eodhd_rsi_backtest -- AAPL.US`

use chrono::{NaiveDate, TimeZone, Utc};
use firequant::backtest::{backtest_simple_bool_signal, PerformanceMetrics};
use firequant::core::indicators::incremental::rsi::RSI;
use firequant::core::traits::Next;
use polars::prelude::*;
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
struct EodBar {
    date: String,
    close: f64,
}

fn main() {
    let token = env::var("EODHD_API_KEY").expect("set EODHD_API_KEY");
    let symbol = env::args().nth(1).unwrap_or_else(|| "AAPL.US".to_string());
    let url = format!(
        "https://eodhd.com/api/eod/{}?api_token={}&fmt=json&period=d&from=2024-01-01",
        symbol, token
    );
    let mut bars: Vec<EodBar> = reqwest::blocking::Client::new()
        .get(&url)
        .send()
        .expect("request")
        .json()
        .expect("json");
    bars.sort_by(|a, b| a.date.cmp(&b.date));

    let mut rsi = RSI::new(14);
    let mut timestamps = Vec::with_capacity(bars.len());
    let mut closes = Vec::with_capacity(bars.len());
    let mut signals = Vec::with_capacity(bars.len());
    for bar in &bars {
        let d = NaiveDate::parse_from_str(&bar.date, "%Y-%m-%d").expect("date");
        timestamps.push(Utc.from_utc_datetime(&d.and_hms_opt(0, 0, 0).unwrap()).timestamp());
        closes.push(bar.close);
        let r = rsi.next(bar.close);
        let sig = if r < 30.0 {
            1.0
        } else if r > 70.0 {
            0.0
        } else {
            0.0
        };
        signals.push(sig);
    }

    let df = DataFrame::new(vec![
        Column::new("timestamp".into(), timestamps),
        Column::new("close".into(), closes),
        Column::new("signal".into(), signals),
    ])
    .unwrap();
    let result = backtest_simple_bool_signal(df, "signal").expect("backtest");
    let m = PerformanceMetrics::from_result(&result);
    println!(
        "{} bars={} trades={} final_equity={:.2}",
        symbol,
        bars.len(),
        m.num_trades,
        m.final_equity
    );
}
