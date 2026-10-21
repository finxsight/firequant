//! Core streaming primitives for firequant.
//!
//! Indicators and regime tooling from upstream QuantWave are ported incrementally;
//! this crate starts with the `Next<T>` contract and warmup tracking.

pub mod streaming;
pub mod traits;
pub mod utils;

pub use utils::RingBuffer;

pub use streaming::{StreamingReadiness, TrackedNext, track};
pub use traits::{IndicatorConfig, Next, SmoothingAlgorithm};
