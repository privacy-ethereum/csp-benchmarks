# OpenVM benchmarks

## Prerequisites

Match the CI helper `.github/actions/install-openvm`:

```bash
rustup toolchain install nightly-2025-08-18-aarch64-apple-darwin \
  --component llvm-tools rustc-dev
rustup override set nightly-2025-08-18-aarch64-apple-darwin

cargo +1.86 install --locked --git https://github.com/openvm-org/openvm.git --tag v1.4.0 cargo-openvm
cargo openvm --version
cargo openvm setup
```

## Running the benchmarks

```bash
cargo bench
```
