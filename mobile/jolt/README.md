# Jolt Mobile Private TX Benchmark

Prototype Mopro/UniFFI wrapper for running the Jolt `private_tx` benchmark on mobile.

The exported function is:

```rust
jolt_prove_private_tx(input_size, compiled_program_path)
```

`input_size` is the Merkle depth. `compiled_program_path` must point to the bundled `private_tx.bin` guest.

The iOS sample app loads `ios/MoproApp/private_tx.bin` from the app bundle and reports only `prove_time_ms`.
