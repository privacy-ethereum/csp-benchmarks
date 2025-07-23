# RISC Zero(zkVM) SHA256 Benchmarks

The benchmark code is from RISC Zero benchmark code:

* https://github.com/risc0/risc0/tree/main/benchmarks

## How to run

1. Install the RISC Zero tools (https://dev.risczero.com/api/zkvm/install)

2. Enter the dir

```console
cd risc0
```

3. Run the bench & measure the RAM usage

```console
cargo run --release -- big-sha2
```
