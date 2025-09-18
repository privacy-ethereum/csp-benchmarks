# Ligetron Benchmarks

Ligetron platform is using a [Ligero ZK proving system](https://eprint.iacr.org/2022/1608.pdf).

## Installation & Test Run

### On OSX

From the root directory:

```bash
cd ligetron
./osx_local_setup.sh
```

## Benchmarking

```bash
cd ../
cargo build --release -p utils
./benchmark.sh
```
