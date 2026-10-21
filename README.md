# rustquant

Lean Rust library for **streaming indicators** and **deterministic order execution**. This repo is a Finxsight fork of ideas and code from [QuantWave](https://github.com/lavs9/quantwave), stripped of Python, Polars plugins, WASM, and notebook-oriented backtest surfaces.

## Crates

| Crate | Role |
|-------|------|
| `rustquant` | Facade re-exporting `core` and `exec` |
| `rustquant-core` | `Next<T>` trait, warmup tracking, shared utilities |
| `rustquant-exec` | Order types, OHLC fills, brackets/OCO, stepping `OrderSim`, stops, live-bridge trait |

## Quick example

```rust
use rustquant::exec::{
    CostModel, ExecutionModel, Order, OrderSim, OrderType, Side, ExecBar,
};
use chrono::Utc;

let exec = ExecutionModel::Simple(CostModel::default());
let mut sim = OrderSim::new(&exec);
let ts = Utc::now();
let bar = ExecBar { open: 100.0, high: 101.0, low: 99.0, close: 100.5 };
sim.step(ts, bar, &[Order::new(Side::Buy, OrderType::Market, 10.0)]);
let trades = sim.finish();
```

## Build

```bash
cargo test --workspace
```

## License

MIT — see [LICENSE](LICENSE). Upstream QuantWave is also MIT; see [docs/ATTRIBUTION.md](docs/ATTRIBUTION.md).

## Roadmap

See [docs/ROADMAP.md](docs/ROADMAP.md).
