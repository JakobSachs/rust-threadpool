# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

A Rust project exploring different thread pool implementations with varying synchronization strategies. Each pool variant experiments with different approaches to task scheduling and thread coordination.

## Build & Run Commands

```bash
# Build
cargo build --release

# Run a specific benchmark (pool size via env var)
POOL_SIZE=4 cargo run --release --bin bench-v1-spinlock

# Run all benchmarks and generate comparison data
./benchmark.sh
gnuplot plot_benchmark.gp
```

## Architecture

### Pool Implementations (`src/pools/`)

Each variant implements a `Pool` struct with this common interface:
- `Pool::new(size: usize)` - Create pool with N worker threads
- `pool.submit(func)` - Submit a single task
- `pool.join_all(self)` - Wait for completion and shut down

**Variants:**
- `v1_spinlock` - Spinlock, for-loop task submission
- `v2_spinlock_batched` - Spinlock, batched submission via `submit_iter()`
- `v3_condvar` - Condvar-based, for-loop task submission
- `v4_condvar_batched` - Condvar-based, batched submission via `submit_iter()`
- `v5_condvar_chunked` - Condvar-based, chunked tasks (fewer tasks via `chunk_size`)

### Benchmark Infrastructure

- `src/bench.rs` - Shared benchmark workload (Collatz sequence computation)
- `src/bin/bench_*.rs` - Per-variant benchmark binaries
- `benchmark.sh` - Auto-discovers and runs all `bench-*` binaries across pool sizes 1-32

## Adding New Pool Implementations

1. Create `src/pools/v{N}_{name}.rs` with `Pool` struct
2. Export in `src/pools/mod.rs`
3. Create `src/bin/bench_v{N}_{name}.rs` benchmark binary
4. Add `[[bin]]` entry to `Cargo.toml` with name pattern `bench-v{N}-{name}`
5. Output format must match: `[NAME T={}] Processed {} numbers in {:.3}s ({:.2} k-numbers/sec)`
