# Miden benchmarks

## Prerequisites

Use the same Rust toolchain that CI applies (`.github/workflows/rust_benchmarks_parallel.yml`):

```bash
rustup toolchain install nightly-2025-08-18-aarch64-apple-darwin \
  --component llvm-tools rustc-dev
rustup override set nightly-2025-08-18-aarch64-apple-darwin
```

## Benchmarking

```bash
cargo bench
```
