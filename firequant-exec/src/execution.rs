//! Execution cost models and shared bar/signal/trade records.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Basic execution cost model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostModel {
    /// Commission in basis points (e.g. 10.0 = 0.10%).
    pub commission_bps: f64,
    /// Slippage in basis points applied to fill price (e.g. 5.0 = 0.05%).
    pub slippage_bps: f64,
    /// Initial cash balance (default 100_000.0).
    pub initial_cash: f64,
}

impl Default for CostModel {
    fn default() -> Self {
        Self {
            commission_bps: 5.0,
            slippage_bps: 2.0,
            initial_cash: 100_000.0,
        }
    }
}

/// Pluggable commission model.
pub trait CommissionModel: Send + Sync + std::fmt::Debug {
    fn calculate_commission(&self, fill_quantity: f64, fill_price: f64) -> f64;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BpsCommissionModel {
    pub bps: f64,
}

impl CommissionModel for BpsCommissionModel {
    fn calculate_commission(&self, fill_quantity: f64, fill_price: f64) -> f64 {
        (fill_quantity.abs() * fill_price) * (self.bps / 10_000.0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FixedPerShareCommissionModel {
    pub per_share: f64,
}

impl CommissionModel for FixedPerShareCommissionModel {
    fn calculate_commission(&self, fill_quantity: f64, _fill_price: f64) -> f64 {
        fill_quantity.abs() * self.per_share
    }
}

/// Pluggable slippage model.
pub trait SlippageModel: Send + Sync + std::fmt::Debug {
    fn apply(&self, price: f64, quantity: f64, is_buy: bool, adv: Option<f64>) -> f64;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BpsSlippageModel {
    pub bps: f64,
}

impl SlippageModel for BpsSlippageModel {
    fn apply(&self, price: f64, _quantity: f64, is_buy: bool, _adv: Option<f64>) -> f64 {
        let s = self.bps / 10_000.0;
        if is_buy {
            price * (1.0 + s)
        } else {
            price * (1.0 - s)
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SquareRootMarketImpactSlippage {
    pub impact_coef: f64,
    pub max_participation: f64,
}

impl SlippageModel for SquareRootMarketImpactSlippage {
    fn apply(&self, price: f64, quantity: f64, is_buy: bool, adv: Option<f64>) -> f64 {
        let adv = adv.unwrap_or(1_000_000.0);
        let part = (quantity.abs() / adv).min(self.max_participation);
        let impact = self.impact_coef * part.sqrt();
        if is_buy {
            price * (1.0 + impact)
        } else {
            price * (1.0 - impact)
        }
    }
}

/// Execution model config: simple bps costs or high-fidelity impact.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExecutionModel {
    Simple(CostModel),
    HighFidelity {
        commission: BpsCommissionModel,
        slippage: SquareRootMarketImpactSlippage,
    },
}

impl Default for ExecutionModel {
    fn default() -> Self {
        ExecutionModel::Simple(CostModel::default())
    }
}

impl ExecutionModel {
    pub fn commission_for(&self, qty: f64, px: f64) -> f64 {
        match self {
            ExecutionModel::Simple(cm) => (qty.abs() * px) * (cm.commission_bps / 10_000.0),
            ExecutionModel::HighFidelity { commission, .. } => {
                commission.calculate_commission(qty, px)
            }
        }
    }

    pub fn slippage_price(&self, price: f64, qty: f64, is_buy: bool, adv: Option<f64>) -> f64 {
        match self {
            ExecutionModel::Simple(cm) => {
                let s = cm.slippage_bps / 10_000.0;
                if is_buy {
                    price * (1.0 + s)
                } else {
                    price * (1.0 - s)
                }
            }
            ExecutionModel::HighFidelity { slippage, .. } => {
                slippage.apply(price, qty, is_buy, adv)
            }
        }
    }
}

/// A completed (or open) trade record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trade {
    pub trade_id: u32,
    pub symbol: Option<String>,
    pub side: i8,
    pub entry_ts: DateTime<Utc>,
    pub entry_price: f64,
    pub entry_fill_price: f64,
    pub exit_ts: Option<DateTime<Utc>>,
    pub exit_price: Option<f64>,
    pub exit_fill_price: Option<f64>,
    pub pnl_gross: f64,
    pub costs: f64,
    pub pnl_net: f64,
    pub quantity: f64,
    pub entry_metadata: Option<HashMap<String, f64>>,
}

/// Per-bar equity snapshot.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EquityPoint {
    pub ts: DateTime<Utc>,
    pub symbol: Option<String>,
    pub equity: f64,
    pub cash: f64,
    pub position: f64,
    pub close: f64,
}

/// Minimal OHLC bar for streaming strategies and live bridge export.
#[derive(Debug, Clone)]
pub struct Bar {
    pub ts: DateTime<Utc>,
    pub close: f64,
    pub high: Option<f64>,
    pub low: Option<f64>,
}

/// Signed exposure signal from a `Next<&Bar, Output = StrategySignal>` strategy.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategySignal {
    pub exposure: f64,
    pub metadata: Option<HashMap<String, f64>>,
}

impl Default for StrategySignal {
    fn default() -> Self {
        Self {
            exposure: 0.0,
            metadata: None,
        }
    }
}
