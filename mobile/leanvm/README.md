# LeanVM Mobile Benchmarks

Prototype Mopro/UniFFI wrapper for running LeanVM benchmarks on iOS.

The exported functions are:

```rust
leanvm_prove_private_tx(input_size)
leanvm_prove_constant_overhead(input_size)
leanvm_prove_merkle_fake(input_size)
leanvm_prove_hash_poseidon16(input_size)
leanvm_prove_merkle_poseidon16(input_size)
```

`input_size` is Merkle depth for `private_tx`, branch count for `merkle_*`, hash count for `hash_*`, and ignored by `constant_overhead`. LeanVM compiles benchmark bytecode from the benchmark crate instead of loading bundled guest binaries.

The iOS sample app lets the user select a target, runs 10 proof samples with 5 seconds between samples, logs every raw sample, and reports mean, median, min, max, stddev, and all samples.
