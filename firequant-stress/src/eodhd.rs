//! Full-history EODHD download (no `from` filter).

use crate::OhlcvBar;
use serde::Deserialize;
use std::time::Duration;

const EOD_URL: &str = "https://eodhd.com/api/eod";

#[derive(Debug, Deserialize)]
struct EodRow {
    date: String,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    volume: f64,
}

pub fn api_token() -> Result<String, String> {
    let t = std::env::var("EODHD_API_KEY")
        .map_err(|_| "EODHD_API_KEY not set".to_string())?
        .trim()
        .to_string();
    if t.is_empty() {
        return Err("EODHD_API_KEY is empty".to_string());
    }
    Ok(t)
}

/// All available daily bars for `symbol` (e.g. `AAPL.US`).
pub fn fetch_full_history(symbol: &str) -> Result<Vec<OhlcvBar>, String> {
    let token = api_token()?;
    let url = format!(
        "{}/{}?api_token={}&fmt=json&period=d",
        EOD_URL,
        symbol,
        token
    );
    let client = reqwest::blocking::Client::builder()
        .user_agent("firequant-stress/0.1")
        .timeout(Duration::from_secs(180))
        .build()
        .map_err(|e| e.to_string())?;
    let rows: Vec<EodRow> = client
        .get(&url)
        .send()
        .map_err(|e| e.to_string())?
        .error_for_status()
        .map_err(|e| e.to_string())?
        .json()
        .map_err(|e| e.to_string())?;
    Ok(rows
        .into_iter()
        .map(|r| OhlcvBar {
            date: r.date,
            open: r.open,
            high: r.high,
            low: r.low,
            close: r.close,
            volume: r.volume,
        })
        .collect())
}
