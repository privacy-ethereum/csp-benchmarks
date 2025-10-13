# STWO(cairo-lang) SHA256 benchmark

The benchmark codes are based on: 
- https://github.com/cartridge-gg/cairo-sha256/blob/main/src/sha256.cairo 
- https://github.com/cartridge-gg/cairo-sha256/blob/main/src/packed_sha256.cairo 
- https://github.com/cartridge-gg/cairo-sha256/blob/main/tests/test_sha256.cairo 

## Installation & Test Run

### On OSX

From the root directory:

```bash
cd stwo
./osx_local_setup.sh
```

## Benchmarking

From the root directory:

```bash
cargo build --release -p utils
./benchmark.sh
```

## References
- https://github.com/cartridge-gg/cairo-sha256
- https://github.com/starkware-libs/stwo-cairo?tab=readme-ov-file#using-stwo-to-prove-cairo-programs
- https://docs.cairo-lang.org/cairozero/quickstart.html
