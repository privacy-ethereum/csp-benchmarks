# ProveKit benchmarks

## Prerequisites

The ProveKit benches rely on Noir tooling to compile the circuits. Install the exact version used in `.github/workflows/rust_benchmarks_parallel.yml`:

```bash
curl -L https://raw.githubusercontent.com/noir-lang/noirup/main/install | bash
export PATH="$HOME/.nargo/bin:$PATH"
~/.nargo/bin/noirup --version 1.0.0-beta.11

rustup toolchain install nightly-2025-08-18-aarch64-apple-darwin \
  --component llvm-tools rustc-dev
rustup override set nightly-2025-08-18-aarch64-apple-darwin
```

## Benchmarking

```bash
cargo bench
```
