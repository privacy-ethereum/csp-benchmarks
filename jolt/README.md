# Jolt benchmarks

## Installation

Use the same steps as `.github/workflows/rust_benchmarks_parallel.yml`:

```bash
rustup toolchain install nightly-2025-08-18-aarch64-apple-darwin \
  --component llvm-tools rustc-dev
rustup override set nightly-2025-08-18-aarch64-apple-darwin
```

## Benchmarking

```bash
cargo bench
```
