# Flock Benchmarks

This crate integrates Flock's SHA-256 compression and Keccak-f1600 permutation
proof paths into the shared benchmark harness.

The shared harness passes the canonical byte-size values used by the other
systems. This crate maps those byte sizes to the number of internal hash
operations a full hash would execute, then adds one extra operation to simulate
fixed padding and wrapper overhead:

- SHA-256 operations: `ceil((input_size + 1 + 8) / 64) + 1`
- Keccak-256 operations: `floor(input_size / 136) + 1 + 1`

The extra `+1` is a conservative over-approximation: one full core is more work
than the true padding-specific constraint overhead, but keeps the benchmark from
under-modeling fixed costs.

Flock exposes these as R1CS proofs for the underlying SHA-256 compression blocks
or Keccak-f1600 permutations. The benchmark rows are tagged as `compressions`
for SHA-256 and `permutations` for Keccak.

Important limitation: this path proves independent cores. It does not bind the
SHA-256 chaining value from one compression block to the next, does not bind
Keccak sponge state across rate blocks, and exposes no public digest. These rows
therefore measure internal core proving throughput at the byte-derived operation
count; they should not be interpreted as proving message-to-digest soundness for
a full hash verifier.
