# Cairo-M(kakarot zkVM) SHA256 benchmarks

This benchmark code is from: https://github.com/kkrt-labs/zkvm-benchmarks/tree/master/cairo-m

## Run SHA256 benches

1. Install the prerequisites for MacOS users
https://github.com/kkrt-labs/cairo-m?tab=readme-ov-file#note-for-macos-users

2. Run the benchmark

```bash
chmod +x ../measure_mem_avg.sh
RUSTFLAGS="-C link-arg=-fuse-ld=/usr/local/bin/ld64.lld -C target-cpu=native" cargo bench 
```
