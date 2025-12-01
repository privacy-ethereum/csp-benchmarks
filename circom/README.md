# Circom SHA256 benchmarks

This benchmark code is from: https://github.com/brevis-network/zk-benchmark/tree/main/circom

## Prerequisites

Use the same toolchain as `.github/workflows/rust_benchmarks_parallel.yml`:

```bash
rustup toolchain install nightly-2025-08-18-aarch64-apple-darwin \
  --component llvm-tools rustc-dev
rustup override set nightly-2025-08-18-aarch64-apple-darwin
```

## Run the benchmarks

```bash
cargo bench
```
