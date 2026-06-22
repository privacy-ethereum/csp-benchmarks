# Binius64 SHA256 benchmarks

These benchmarks use official circuits from the Binius64 project: https://github.com/IrreducibleOSS/binius64/blob/main/prover/examples/src/circuits/sha256.rs

## Prerequisites

Use the pinned toolchain from `rust-toolchain.toml`:

```bash
rustup toolchain install 1.95.0
```

## Run the benchmarks

```bash
RUSTFLAGS="-C target-cpu=native" cargo bench
```
