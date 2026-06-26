# Flock Benchmarks

This crate integrates SHA-256 and Keccak-256 full hash proof paths built on
Flock's upstream hash cores into the shared benchmark harness.

The shared harness passes the canonical byte size values used by the other
systems.

SHA-256 uses a full hash wrapper assembled from Flock's upstream fast batched
compression layout: one compression per outer slot, upstream fused witness
generation, and the upstream fixed `K_LOG`. The wrapper adds separate checks for
SHA padding and length constants, chaining values between compressions, and
public digest openings.

Keccak-256 uses Flock's upstream Keccak-f1600 permutation walker, with an added
sponge relation layer over the same fast batched core layout: one permutation
per outer slot, upstream fused witness generation, and the upstream fixed
`K_LOG`. The wrapper constrains capacity carryover, fixed padding bits, and the
final padded block, then opens the packed cells in the final state that contain
the digest. Because Flock's state layout is contiguous by lane, those public
openings can reveal additional bits from the final state in the same cells. Flock
is recorded as non-ZK in the benchmark metadata, so this is acceptable for these
rows.

The reported `preprocessing_size` is the serialized public Flock setup artifact:
hash kind, setup shape derived from the byte input, PCS params, statement
digest, and the upstream `BlockR1cs` layout. For SHA-256 this includes the
materialized sparse R1CS matrices. For Keccak-256, upstream Flock uses a
hardcoded permutation lincheck walker with an empty R1CS matrix stub, so the
serialized setup is smaller than a system that serializes all Keccak constraints
as table data.

The reported `num_constraints` is bit level R1CS row count over Flock's binary
field. This is the unit Flock exposes, but it is not directly comparable to
word level constraint counts reported by systems such as Binius64.
