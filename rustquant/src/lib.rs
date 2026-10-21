//! rustquant — lean Rust quant execution and streaming primitives.
//!
//! Re-exports the workspace crates. See `rustquant-exec` for order simulation and
//! `rustquant-core` for the `Next<T>` indicator contract.

pub use rustquant_core as core;
pub use rustquant_exec as exec;
