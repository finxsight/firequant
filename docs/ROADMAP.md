# firequant roadmap

## Shipped (QuantWave parity — Rust only)

| Area | Crate | Status |
|------|-------|--------|
| ~225 indicators, Ehlers DSP, PA, patterns | `firequant-core` | Ported from QuantWave |
| Gold-standard / integration tests | `firequant-core/tests` | Ported |
| Polars `.ta` expression plugins | `firequant-polars` | Ported |
| Vectorized + streaming backtest, metrics, MC, walk-forward, TPE | `firequant-backtest` | Ported |
| Order stepping API (compat) | `firequant-exec` | Re-exports backtest execution surface |
| Facade + optional Polars | `firequant` | `default = ["polars"]` |

**Not ported (by design):** Python (`quantwave-py`), WASM (`quantwave-wasm`), xtask codegen, mkdocs site, PyPI wheels.

## Toolchain

`rust-toolchain.toml` pins **nightly** (matches QuantWave dependency MSRV for Polars 0.46 / nalgebra 0.35).

## Next (FIRE-specific)

1. Rebrand docs/examples; remove upstream task IDs from comments over time.
2. Finxsight bar feeds + `LiveBridge` broker adapters.
3. Publish to crates.io as `firequant-*` when API stabilizes.
4. Slim CI: nextest + cached Polars builds.
