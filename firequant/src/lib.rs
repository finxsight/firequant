//! firequant — lean Rust quant execution and streaming primitives.
//!
//! Re-exports the workspace crates. See `firequant-exec` for order simulation and
//! `firequant-core` for the `Next<T>` indicator contract.

pub use firequant_core as core;
pub use firequant_exec as exec;
