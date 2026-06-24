# Stark-V Mobile Private TX Benchmark

Prototype Mopro/UniFFI wrapper for running the Stark-V `private_tx` benchmark on mobile.

The exported function is:

```rust
stark_v_prove_private_tx(input_size, compiled_program_path)
```

`input_size` is the Merkle depth. `compiled_program_path` must point to the bundled `private_tx.bin` guest.

The iOS sample app loads `ios/MoproApp/private_tx.bin` from the app bundle, runs 10 proof samples with 5 seconds between samples, logs every raw sample, and reports `prove_time_ms` as the mean plus `samples_ms`.
