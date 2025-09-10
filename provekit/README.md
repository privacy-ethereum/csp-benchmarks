# ProveKit SHA256 Benchmark

This benchmark code is using the ProveKit of World Foundation(https://github.com/worldfnd/ProveKit).


## Benchmarking

```bash
cargo bench
```

Measure RAM footprint:

```bash
chmod +x ../measure_mem_avg.sh
cargo run --release --bin measure_mem
```
