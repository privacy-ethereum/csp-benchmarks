# SP1 benchmarks

## Prerequisites

Provision the SP1 toolchain exactly like `.github/actions/install-sp1`:

```bash
curl -fsSL --retry 5 --proto '=https' --tlsv1.2 https://sp1up.succinct.xyz | bash
export PATH="$HOME/.sp1/bin:$PATH"

# Requires a GitHub token with `packages:read` scopes.
export GITHUB_TOKEN=<your-token>
sp1up -v v5.2.1 --token "$GITHUB_TOKEN"

cargo-prove prove --version

rustup toolchain install nightly-2025-08-18-aarch64-apple-darwin \
  --component llvm-tools rustc-dev
rustup override set nightly-2025-08-18-aarch64-apple-darwin
```

## Running the benchmarks

```bash
cargo bench
```
