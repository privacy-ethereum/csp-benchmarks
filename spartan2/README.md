# Spartan2 SHA256 Benchmarks

This crate implements SHA256 hash verification benchmarks using Spartan2.

## Overview

This benchmark uses the official Spartan2 implementation from Microsoft to prove SHA256 hash computations. The circuit is implemented using bellpepper gadgets directly (no Circom compilation needed).

## Running Benchmarks

```bash
# Quick test with reduced inputs
BENCH_INPUT_PROFILE=reduced cargo bench -p spartan2-bench --bench sha256

# Full benchmark
BENCH_INPUT_PROFILE=full cargo bench -p spartan2-bench --bench sha256

# Test the memory measurement binary
cargo run --release --bin sha256_mem_spartan2
```

## Circuit Details

The SHA256 circuit computes the SHA256 hash of a variable-length message. It takes as input:
- `preimage`: The message bytes to hash (variable length, typically 128-2048 bytes for benchmarks)

The circuit outputs:
- `hash`: The 256-bit SHA256 hash of the preimage (exposed as public values)

## Implementation Notes

- Uses `T256HyraxEngine` (P256 field with Hyrax polynomial commitment scheme)
- Circuit implementation uses `bellpepper::gadgets::sha256::sha256` directly
- Based on the official Spartan2 example: https://github.com/microsoft/Spartan2/blob/main/examples/sha256.rs
- Uses `SpartanSNARK` (not `SpartanZkSNARK`) as per the official example
- Benchmark harness follows the standard CSP benchmarks pattern
