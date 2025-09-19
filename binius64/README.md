# Binius64 SHA256 benchmarks

These benchmarks use official circuits from the Binius64 project: https://github.com/IrreducibleOSS/binius64/blob/main/prover/examples/src/circuits/sha256.rs

# Run SHA256 benches

```bash
chmod +x ../measure_mem_avg.sh
RUSTFLAGS="-C target-cpu=native" cargo bench
```
