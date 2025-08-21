# Binius SHA256 benchmarks

These benchmarks use official circuits from the Binius project: https://github.com/IrreducibleOSS/binius/blob/main/examples/sha256_circuit.rs and https://github.com/IrreducibleOSS/binius/blob/main/examples/sha256_circuit_with_lookup.rs.

# Run SHA256 benches

```bash
cargo bench
```

# Measure SHA256 RAM footprint

```bash
chmod +x ../measure_mem_avg.sh
../measure_mem_avg.sh --json sha2_binius_lookup_mem_report.json -- cargo r -r --bin measure_lookup_mem
../measure_mem_avg.sh --json sha2_binius_no_lookup_mem_report.json -- cargo r -r --bin measure_no_lookup_mem 
```
