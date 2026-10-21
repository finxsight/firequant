# firequant

FIRE / Finxsight **Rust quant toolkit** — indicators, regimes, options analytics, Polars TA plugins, and an execution-aware backtest engine. Functionally aligned with [QuantWave](https://github.com/lavs9/quantwave) **without** Python, WASM, or PyPI packaging.

## Crates

| Crate | Role |
|-------|------|
| `firequant` | Facade: `core`, `backtest`, `exec`, optional `polars` |
| `firequant-core` | Indicators, features, regimes, `Next<T>` streaming |
| `firequant-backtest` | Polars batch + streaming sim, metrics, walk-forward, Monte Carlo |
| `firequant-polars` | Polars expression plugins (`ta`, `bt` namespaces) |
| `firequant-exec` | Thin re-export of order/fill types and `OrderSim` |

## Toolchain

Requires **Rust nightly** (see `rust-toolchain.toml`).

```bash
cargo test --workspace
```

Live EODHD smoke (needs `EODHD_API_KEY`):

```bash
cargo test -p firequant --test eodhd_live -- --nocapture
cargo run -p firequant --example eodhd_rsi_backtest -- MSFT.US
```

## Quick example (order sim)

```rust
use firequant::exec::{
    CostModel, ExecutionModel, ExecBar, Order, OrderSim, OrderType, Side,
};

let exec = ExecutionModel::Simple(CostModel::default());
let mut sim = OrderSim::new(&exec);
// ... step bars, then sim.finish()
```

## Polars

```rust
use firequant::polars; // registers expression plugins when linked
```

## License

MIT — upstream QuantWave is MIT; see [docs/ATTRIBUTION.md](docs/ATTRIBUTION.md).
