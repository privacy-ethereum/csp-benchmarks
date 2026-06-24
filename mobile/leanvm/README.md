# LeanVM Mobile Private TX Benchmark

Prototype Mopro/UniFFI wrapper for running the LeanVM `private_tx` benchmark on mobile.

The exported function is:

```rust
leanvm_prove_private_tx(input_size)
```

`input_size` is the Merkle depth. LeanVM compiles the benchmark program from the benchmark crate instead of loading a bundled guest binary.

The iOS sample app runs 10 proof samples with 5 seconds between samples, logs every raw sample, and reports `prove_time_ms` as the mean plus `samples_ms`.
