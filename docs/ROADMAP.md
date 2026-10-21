# rustquant roadmap

## Done (v0.1 bootstrap)

- Workspace split: `rustquant-core`, `rustquant-exec`, facade `rustquant`
- Order-driven stepping simulator with batch/stream parity tests (from QuantWave)
- Stops evaluation primitives and `LiveBridge` trait stub
- No Python / Polars / WASM / xtask / PyPI packaging

## Next

1. **Signal path** — `Next<&Bar, StrategySignal>` exposure simulator wired to `ExecutionModel` + `StopConfig` (port streaming slice from upstream backtest without DataFrames).
2. **Indicators** — port indicators on demand behind `Next<f64>` (start with smoothing: SMA/EMA), gold tests optional per indicator.
3. **Multi-symbol** — shared-capital portfolio stepping without Polars (columnar or iterator feeds).
4. **Finxsight integration** — bar feed from exchange_data / FIRE dashboards; broker adapter implementing `LiveBridge`.
5. **CI** — `cargo clippy`, `cargo test`, deny `unsafe`.

## Explicitly out of scope

- Polars expression plugins and `.ta` / `.bt` namespaces
- Python wheels and `maturin`
- Options India / regime HMM suites unless needed for a concrete product feature
- HTML tear sheets and walk-forward optimizers in v0.x (revisit as separate crates if needed)
