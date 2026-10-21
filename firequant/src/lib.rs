//! firequant — FIRE quant toolkit (indicators, backtest, optional Polars).
//!
//! Adapted from [QuantWave](https://github.com/lavs9/quantwave); Rust-only, no Python/WASM.

pub use firequant_core as core;
pub use firequant_backtest as backtest;
pub use firequant_exec as exec;

#[cfg(feature = "polars")]
pub use firequant_polars as polars;
