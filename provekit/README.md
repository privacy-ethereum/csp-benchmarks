# ProveKit SHA256 Benchmark

This benchmark code is using the ProveKit of World Foundation(https://github.com/worldfnd/ProveKit).


## Benchmarking

```bash
cargo bench
```

Measure some metrics(e.g. proof size):

```bash
cargo run -r --bin provekit-measure
```

Measure RAM footprint:
```bash
chmod +x ../measure_mem_avg.sh
../measure_mem_avg.sh --json sha2_provekit_mem_report.json -- cargo r -r --bin provekit-measure-mem
```
