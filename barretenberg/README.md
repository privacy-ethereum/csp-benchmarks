# Noir + Barretenberg benchmarks

This benchmark code is from: https://github.com/noir-lang/sha256/tree/main.
Bench props are from:

- https://deepwiki.com/AztecProtocol/barretenberg#proving-systems
- https://deepwiki.com/AztecProtocol/barretenberg/6.1-ultrahonk

## Prerequisites (macOS arm64)

Match the CI setup from `.github/workflows/sh_benchmarks_parallel.yml` and the helper actions under `.github/actions`:

1. **Install Noir / Nargo**

   ```bash
   curl -L https://raw.githubusercontent.com/noir-lang/noirup/main/install | bash
   export PATH="$HOME/.nargo/bin:$PATH"
   ~/.nargo/bin/noirup --version 1.0.0-beta.13
   ```

2. **Install the Barretenberg CLI**

   ```bash
   curl -L https://raw.githubusercontent.com/AztecProtocol/aztec-packages/refs/heads/next/barretenberg/bbup/install | bash
   export PATH="$HOME/.bb:$PATH"
   bbup -v 0.87.0
   ```

## Installation & Test Run

### On OSX

From the root directory:

```bash
cd barretenberg
./osx_local_setup.sh
```

## Benchmarking

```bash
cd ../
cargo build --release -p utils
./benchmark.sh
```
