# firequant-stress

Downloads **full daily EOD history** from EODHD for 100 US tickers, caches JSON on disk (gitignored), and runs five quant stress functions via `firequant-core` / `firequant-backtest`.

## Setup

```bash
export EODHD_API_KEY=...   # never commit; see repo .env.example
```

Cache directory (default): `./.cache/eodhd/` under the firequant repo root, or set `FIREQUANT_EODHD_CACHE`.

## Commands

```bash
cargo run -p firequant-stress --bin eodhd_stress -- fetch   # download missing symbols
cargo run -p firequant-stress --bin eodhd_stress -- stress  # run 5-function bed on cache
cargo run -p firequant-stress --bin eodhd_stress -- all     # fetch then stress
```

## Stress bed (5 functions)

1. RSI(14) on close  
2. EMA(20) on close  
3. SMA(50) on close  
4. ATR(14) on (high, low, close)  
5. RSI mean-reversion backtest (Polars batch path)
