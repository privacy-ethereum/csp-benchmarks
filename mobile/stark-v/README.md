# Stark-V Mobile Benchmarks

Prototype Mopro/UniFFI wrapper for running Stark-V benchmarks on iOS.

The exported functions are:

```rust
stark_v_prove_private_tx(input_size, compiled_program_path)
stark_v_prove_constant_overhead(input_size, compiled_program_path)
stark_v_prove_merkle_fake(input_size, compiled_program_path)
stark_v_prove_hash_sha256(input_size, compiled_program_path)
stark_v_prove_merkle_sha256(input_size, compiled_program_path)
stark_v_prove_hash_keccak(input_size, compiled_program_path)
stark_v_prove_merkle_keccak(input_size, compiled_program_path)
```

`input_size` is Merkle depth for `private_tx`, branch count for `merkle_*`, hash count for `hash_*`, and ignored by `constant_overhead`. `compiled_program_path` must point to the matching bundled `<target>.bin` guest.

The iOS sample app lets the user select a target, loads the matching `.bin` from the app bundle, runs 10 proof samples with 5 seconds between samples, logs every raw sample, and reports mean, median, min, max, stddev, and all samples.
