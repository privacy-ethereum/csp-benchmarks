# Nexus benchmarks

## Prerequisites

```bash
rustup toolchain install nightly-2025-04-06-aarch64-apple-darwin \
  --component llvm-tools rustc-dev rust-src
rustup override set nightly-2025-04-06-aarch64-apple-darwin
```

## Running the benchmarks

```bash
cargo bench
```
