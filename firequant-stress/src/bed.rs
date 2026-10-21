//! Five-function stress bed over cached histories.

use crate::TickerSeries;
use chrono::{NaiveDate, TimeZone, Utc};
use firequant_backtest::{backtest_simple_bool_signal, PerformanceMetrics};
use firequant_core::indicators::incremental::rsi::RSI;
use firequant_core::indicators::smoothing::{EMA, SMA};
use firequant_core::indicators::volatility::ATR;
use firequant_core::traits::Next;
use polars::prelude::*;
use std::time::Instant;

#[derive(Debug, Clone)]
pub struct StressBedResult {
    pub symbol: String,
    pub bars: usize,
    pub last_rsi: f64,
    pub last_ema: f64,
    pub last_sma: f64,
    pub last_atr: f64,
    pub backtest_trades: f64,
    pub backtest_final_equity: f64,
}

#[derive(Debug)]
pub struct StressReport {
    pub results: Vec<StressBedResult>,
    pub total_bars: usize,
    pub elapsed_ms: u128,
    pub failures: Vec<String>,
}

pub fn run_stress_bed(series_list: &[TickerSeries]) -> StressReport {
    let start = Instant::now();
    let mut results = Vec::with_capacity(series_list.len());
    let mut failures = Vec::new();
    let mut total_bars = 0usize;

    for series in series_list {
        if series.bars.len() < 60 {
            failures.push(format!(
                "{}: only {} bars (need >= 60)",
                series.symbol,
                series.bars.len()
            ));
            continue;
        }
        match run_one(series) {
            Ok(r) => {
                total_bars += r.bars;
                results.push(r);
            }
            Err(e) => failures.push(format!("{}: {}", series.symbol, e)),
        }
    }

    StressReport {
        results,
        total_bars,
        elapsed_ms: start.elapsed().as_millis(),
        failures,
    }
}

fn run_one(series: &TickerSeries) -> Result<StressBedResult, String> {
    let closes = series.closes();
    let (high, low, close) = series.hlc();

    let mut rsi = RSI::new(14);
    let mut ema = EMA::new(20);
    let mut sma = SMA::new(50);
    let mut atr = ATR::new(14);

    let mut last_rsi = f64::NAN;
    let mut last_ema = f64::NAN;
    let mut last_sma = f64::NAN;
    let mut last_atr = f64::NAN;

    for i in 0..closes.len() {
        last_rsi = rsi.next(closes[i]);
        last_ema = ema.next(closes[i]);
        last_sma = sma.next(closes[i]);
        last_atr = atr.next((high[i], low[i], close[i]));
    }

    if !last_rsi.is_finite() {
        return Err(format!("RSI not finite: {}", last_rsi));
    }

    let signals: Vec<f64> = {
        let mut rsi2 = RSI::new(14);
        closes
            .iter()
            .map(|c| {
                let r = rsi2.next(*c);
                if r < 30.0 {
                    1.0
                } else if r > 70.0 {
                    0.0
                } else {
                    0.0
                }
            })
            .collect()
    };

    let timestamps: Vec<i64> = series
        .bars
        .iter()
        .map(|b| {
            let d = NaiveDate::parse_from_str(&b.date, "%Y-%m-%d")
                .map_err(|e| e.to_string())?;
            Ok(Utc.from_utc_datetime(&d.and_hms_opt(0, 0, 0).unwrap()).timestamp())
        })
        .collect::<Result<Vec<_>, String>>()?;

    let df = DataFrame::new(vec![
        Column::new("timestamp".into(), timestamps),
        Column::new("close".into(), closes.clone()),
        Column::new("signal".into(), signals),
    ])
    .map_err(|e| e.to_string())?;

    let bt = backtest_simple_bool_signal(df, "signal").map_err(|e| e.to_string())?;
    let metrics = PerformanceMetrics::from_result(&bt);

    Ok(StressBedResult {
        symbol: series.symbol.clone(),
        bars: series.bars.len(),
        last_rsi,
        last_ema,
        last_sma,
        last_atr,
        backtest_trades: metrics.num_trades,
        backtest_final_equity: metrics.final_equity,
    })
}
