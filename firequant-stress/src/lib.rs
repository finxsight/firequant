//! EODHD cache + stress bed shared library.

pub mod bed;
pub mod cache;
pub mod eodhd;
pub mod tickers;

pub use bed::{StressBedResult, StressReport, run_stress_bed};
pub use cache::{
    CacheManifest, cache_dir, entry_from_series, load_manifest, load_series, save_series,
    write_manifest,
};
pub use eodhd::fetch_full_history;
pub use tickers::{TICKER_COUNT, TICKERS};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OhlcvBar {
    pub date: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TickerSeries {
    pub symbol: String,
    pub bars: Vec<OhlcvBar>,
}

impl TickerSeries {
    pub fn closes(&self) -> Vec<f64> {
        self.bars.iter().map(|b| b.close).collect()
    }

    pub fn hlc(&self) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
        let mut h = Vec::with_capacity(self.bars.len());
        let mut l = Vec::with_capacity(self.bars.len());
        let mut c = Vec::with_capacity(self.bars.len());
        for b in &self.bars {
            h.push(b.high);
            l.push(b.low);
            c.push(b.close);
        }
        (h, l, c)
    }
}
