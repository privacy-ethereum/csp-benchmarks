# Binius SHA256 benchmarks

These benchmarks use official circuits from the Binius project: https://github.com/IrreducibleOSS/binius/blob/main/examples/sha256_circuit.rs and https://github.com/IrreducibleOSS/binius/blob/main/examples/sha256_circuit_with_lookup.rs.

# Run SHA256 benches

```bash
chmod +x ../measure_mem_avg.sh
RUSTFLAGS="-C target-cpu=native" cargo bench
```
