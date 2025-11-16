# Noir + Barretenberg SHA256 benchmarks

This benchmark code is from: https://github.com/noir-lang/sha256/tree/main.

Bench props are from:

- https://deepwiki.com/AztecProtocol/barretenberg#proving-systems
- https://deepwiki.com/AztecProtocol/barretenberg/6.1-ultrahonk

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
