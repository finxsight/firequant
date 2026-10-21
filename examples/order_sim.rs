//! Run with: cargo run -p rustquant --example order_sim

use chrono::TimeZone;
use rustquant::exec::{
    CostModel, ExecBar, ExecutionModel, Order, OrderSim, OrderType, Side,
};

fn main() {
    let exec = ExecutionModel::Simple(CostModel {
        commission_bps: 0.0,
        slippage_bps: 0.0,
        initial_cash: 100_000.0,
    });
    let mut sim = OrderSim::new(&exec);
    let ts = chrono::Utc.with_ymd_and_hms(2026, 1, 15, 9, 30, 0).unwrap();
    let bar = ExecBar {
        open: 100.0,
        high: 101.0,
        low: 99.5,
        close: 100.25,
    };
    sim.step(
        ts,
        bar,
        &[Order::new(Side::Buy, OrderType::Market, 100.0)],
    );
    let trades = sim.finish();
    println!("closed trades: {}", trades.len());
    for t in &trades {
        println!(
            "trade {} side {} pnl_net {:.2}",
            t.trade_id,
            t.side,
            t.pnl_net
        );
    }
}
