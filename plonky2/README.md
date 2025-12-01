# Plonky2 SHA256 Benchmark

The circuit is taken from https://github.com/polymerdao/plonky2-sha256/tree/main

> [!NOTE]  
> The custom dependency for `plonky2_u32` is necessary because the original outdated `plonky2_u32` depended on an old version of `plonky2` and did not support serialization, so it would be impossible to measure PK/VK/proof size. Awaiting PR merge (https://github.com/0xPolygonZero/plonky2-u32/pull/7)

## Prerequisites

Use the same toolchain as `.github/workflows/rust_benchmarks_parallel.yml`:

```bash
rustup toolchain install nightly-2025-08-18-aarch64-apple-darwin \
  --component llvm-tools rustc-dev
rustup override set nightly-2025-08-18-aarch64-apple-darwin
```

## Benchmarking

```bash
cargo bench
```
