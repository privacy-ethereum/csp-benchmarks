# Cairo-M(kakarot zkVM) SHA256 benchmarks

This benchmark code is from: https://github.com/kkrt-labs/zkvm-benchmarks/tree/master/cairo-m

## Prerequisites

Follow the same toolchain setup that CI applies in `.github/workflows/rust_benchmarks_parallel.yml`:

```bash
brew install llvm lld
export CC=/opt/homebrew/opt/llvm/bin/clang
export CXX=/opt/homebrew/opt/llvm/bin/clang++
export AR=/opt/homebrew/opt/llvm/bin/llvm-ar
export RANLIB=/opt/homebrew/opt/llvm/bin/llvm-ranlib

rustup toolchain install nightly-2025-04-06-aarch64-apple-darwin \
  --component llvm-tools rustc-dev rust-src
rustup override set nightly-2025-04-06-aarch64-apple-darwin
```

## Run the benchmark

```bash
RUSTFLAGS="-C link-arg=-fuse-ld=lld -C target-cpu=native" cargo bench
```
