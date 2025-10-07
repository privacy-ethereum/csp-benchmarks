# Noir SHA256 benchmarks

This benchmark code is from: https://github.com/worldfnd/provekit/tree/main/noir-examples/noir-native-sha256

## Installation & Test Run

### On OSX

From the root directory:

```bash
cd noir
./osx_local_setup.sh
```

## Benchmarking

```bash
cd ../
cargo build --release -p utils
./benchmark.sh
```
