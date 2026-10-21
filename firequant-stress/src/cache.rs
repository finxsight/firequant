//! On-disk JSON cache per symbol + manifest.

use crate::{OhlcvBar, TickerSeries};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

const MANIFEST: &str = "manifest.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheManifest {
    pub updated_unix: u64,
    pub symbols: Vec<SymbolEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolEntry {
    pub symbol: String,
    pub bars: usize,
    pub first_date: Option<String>,
    pub last_date: Option<String>,
}

pub fn cache_dir() -> PathBuf {
    std::env::var("FIREQUANT_EODHD_CACHE")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("..")
                .join(".cache")
                .join("eodhd")
        })
}

fn symbol_path(dir: &Path, symbol: &str) -> PathBuf {
    let safe = symbol.replace('.', "_");
    dir.join(format!("{safe}.json"))
}

pub fn load_series(dir: &Path, symbol: &str) -> Option<TickerSeries> {
    let path = symbol_path(dir, symbol);
    if !path.is_file() {
        return None;
    }
    let raw = fs::read_to_string(&path).ok()?;
    serde_json::from_str(&raw).ok()
}

pub fn save_series(dir: &Path, series: &TickerSeries) -> std::io::Result<()> {
    fs::create_dir_all(dir)?;
    let path = symbol_path(dir, &series.symbol);
    let json = serde_json::to_string(series)?;
    fs::write(path, json)
}

pub fn write_manifest(dir: &Path, entries: &[SymbolEntry]) -> std::io::Result<()> {
    fs::create_dir_all(dir)?;
    let updated_unix = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let manifest = CacheManifest {
        updated_unix,
        symbols: entries.to_vec(),
    };
    let path = dir.join(MANIFEST);
    fs::write(path, serde_json::to_string_pretty(&manifest)?)
}

pub fn load_manifest(dir: &Path) -> Option<CacheManifest> {
    let path = dir.join(MANIFEST);
    if !path.is_file() {
        return None;
    }
    let raw = fs::read_to_string(path).ok()?;
    serde_json::from_str(&raw).ok()
}

pub fn entry_from_series(series: &TickerSeries) -> SymbolEntry {
    SymbolEntry {
        symbol: series.symbol.clone(),
        bars: series.bars.len(),
        first_date: series.bars.first().map(|b| b.date.clone()),
        last_date: series.bars.last().map(|b| b.date.clone()),
    }
}

pub fn bars_from_eodhd(symbol: &str, raw: Vec<OhlcvBar>) -> TickerSeries {
    let mut bars = raw;
    bars.sort_by(|a, b| a.date.cmp(&b.date));
    TickerSeries {
        symbol: symbol.to_string(),
        bars,
    }
}
