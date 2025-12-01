# RISC Zero benchmarks

## Prerequisites

Follow `.github/actions/install-risc0` to provision the vendor toolchain:

```bash
curl -fsSL --retry 5 --proto '=https' --tlsv1.2 https://risczero.com/install | bash
export PATH="$HOME/.risc0/bin:$PATH"

rzup install rust 1.88.0
rzup install cpp 2024.1.5
rzup install r0vm 3.0.3
rzup install cargo-risczero 3.0.3

rustup toolchain install nightly-2025-08-18-aarch64-apple-darwin \
  --component llvm-tools rustc-dev
rustup override set nightly-2025-08-18-aarch64-apple-darwin
```

## Running the benchmarks

```bash
cargo bench
```
