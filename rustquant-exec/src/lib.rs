//! Deterministic order execution and stepping simulation.
//!
//! Adapted from [QuantWave](https://github.com/lavs9/quantwave) `quantwave-backtest`
//! order path — Polars, Python, WASM, and batch DataFrame engines are intentionally
//! out of scope here.

mod execution;
mod live_bridge;
mod order_exec;
mod orders;
mod stops;

pub use execution::{
    Bar, BpsCommissionModel, BpsSlippageModel, CommissionModel, CostModel, EquityPoint,
    ExecutionModel, FixedPerShareCommissionModel, SlippageModel, SquareRootMarketImpactSlippage,
    StrategySignal, Trade,
};
pub use live_bridge::{LiveBridge, LiveBridgeError, LiveSignalEvent, RecordingLiveBridge};
pub use order_exec::{OrderSim, run_order_simulation};
pub use orders::{
    Bracket, ExecBar, ExitLeg, Fill, FillKind, Order, OrderType, Side, fill_order,
    resolve_bracket,
};
pub use stops::{
    OhlcBar, StopConfig, StopEvaluationMode, StopExit, StopExitKind, StopPositionState,
    evaluate_stops,
};
