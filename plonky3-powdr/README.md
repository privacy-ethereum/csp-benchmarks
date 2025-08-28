# Plonky3 Powdr SHA256 Benchmark

This benchmark code is based on https://github.com/babybear-labs/benchmark/tree/main/powdr/sha.

## Usage

This will run the host and generate ZK proofs.

```bash
cargo run -r --bin plonky3-powdr-sha
```

## Benchmarking

```bash
cargo bench
```

Measure RAM footprint:
```bash
chmod +x ../measure_mem_avg.sh
../measure_mem_avg.sh --json sha256_2048_powdr_mem_report.json -- cargo r -r --bin plonky3-powdr-measure-mem
```
