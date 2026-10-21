//! Stepping order simulation and execution types.
//!
//! Full portfolio backtest, metrics, and Polars batch paths live in `firequant-backtest`.

pub use firequant_backtest::{
    Bar, BpsCommissionModel, BpsSlippageModel, CommissionModel, CostModel, EquityPoint,
    ExecutionDelay, ExecutionModel, FixedPerShareCommissionModel, LiveBridge, LiveBridgeError,
    LiveSignalEvent, Order, OrderSim, OrderType, RecordingLiveBridge, Side, SlippageModel,
    SquareRootMarketImpactSlippage, StrategySignal, Trade, ExecBar, ExitLeg, Fill, FillKind,
    StopConfig, StopEvaluationMode, fill_order, resolve_bracket, run_order_simulation,
};
