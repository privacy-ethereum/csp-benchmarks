# Cairo-M(kakarot zkVM) SHA256 benchmarks

This benchmark code is from: https://github.com/kkrt-labs/zkvm-benchmarks/tree/master/cairo-m

## Run SHA256 benches

```bash
chmod +x ../measure_mem_avg.sh
RUSTFLAGS="-C link-arg=-fuse-ld=/opt/homebrew/bin/ld64.lld -C target-cpu=native" cargo bench 
```
