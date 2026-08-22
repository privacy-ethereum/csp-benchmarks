# Circom SHA256 benchmarks

This benchmark code is from: https://github.com/brevis-network/zk-benchmark/tree/main/circom

## Prerequisites

Use the same toolchain as `.github/workflows/rust_benchmarks_parallel.yml`:

```bash
rustup toolchain install nightly-2025-08-18-aarch64-apple-darwin \
  --component llvm-tools rustc-dev
rustup override set nightly-2025-08-18-aarch64-apple-darwin
```

The ECDSA circuit also needs [circom](https://github.com/iden3/circom) 2.2.2 on PATH. Its witness
generator is 57 MiB of generated C++ — roughly three times the largest artifact stored here — so
`build.rs` compiles it from `ecdsa_32.circom` on demand rather than keeping it in the repo. Every
other circuit ships its `.cpp` and `.dat` in tree and needs nothing installed. The first build
after a clean checkout spends about five minutes in circom before the C++ compile starts.

## Run the benchmarks

```bash
cargo bench
```
