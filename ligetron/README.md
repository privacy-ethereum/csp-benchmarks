# Ligetron Benchmarks

[Ligetron platform](https://github.com/ligeroinc/ligero-prover) is using a [Ligero ZK proving system](https://eprint.iacr.org/2022/1608.pdf).

## Installation & Test Run

### On OSX

From the root directory:

```bash
cd ligetron
./osx_local_setup.sh
```

## Running the Benchmark

From the root directory:

```bash
cargo build --release -p utils
./benchmark.sh
```
