# Binius64 SHA256 benchmarks

These benchmarks use official circuits from the Binius64 project: https://github.com/IrreducibleOSS/binius64/blob/main/prover/examples/src/circuits/sha256.rs

## Prerequisites

Use the same toolchain as `.github/workflows/rust_benchmarks_parallel.yml`:

```bash
rustup toolchain install nightly-2025-08-18-aarch64-apple-darwin \
  --component llvm-tools rustc-dev
rustup override set nightly-2025-08-18-aarch64-apple-darwin
```

## Run the benchmarks

```
RUSTFLAGS="-C target-cpu=native" cargo bench
```
