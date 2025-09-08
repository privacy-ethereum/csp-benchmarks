# Polyhedra Expander SHA256 benchmark

## How to run

```
cd polyhedra-expander
cargo bench
```

# Measure SHA256 RAM footprint

```bash
chmod +x ../measure_mem_avg.sh
../measure_mem_avg.sh --json sha256_2048_polyhedra_expander_mem_report.json -- cargo r -r --bin measure
```
