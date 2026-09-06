# Halo2 SHA256 + Keccak benchmarks

The circuits are Axiom's `zkevm-hashes` crate: https://github.com/axiom-crypto/halo2-lib/tree/develop/hashes/zkevm
(Axiom's revision of the PSE zkEVM hash circuits).

Proved with halo2's KZG backend over BN254 — SHPLONK multi-open, Blake2b for Fiat-Shamir —
which is the configuration the upstream tests use.

> [!NOTE]
> The git dependencies must stay on `branch = "develop"` and must not be pinned to a `rev`.
> `zkevm-hashes` pulls in snark-verifier, which depends on `halo2-base` via that same branch;
> Cargo keys a git source on the ref you name, so a `rev` produces two copies of `halo2-base`
> and the build fails on an `OptimizedPoseidonSpec` type mismatch. Upstream reconciles this
> with a `[patch]` table, which Cargo only honours from the root manifest of a build and so
> does not reach us. The commit is still pinned, by `Cargo.lock`.

## Prerequisites

Use the same toolchain as `.github/workflows/rust_benchmarks_parallel.yml`:

```bash
rustup toolchain install nightly-2026-03-04 --component llvm-tools rustc-dev
rustup override set nightly-2026-03-04
```

## Benchmarking

```bash
# Quick test with reduced inputs
BENCH_INPUT_PROFILE=reduced cargo bench -p halo2_circuits

# Single target
BENCH_INPUT_PROFILE=reduced cargo bench -p halo2_circuits --bench sha256

# Memory measurement binaries
cargo run --release --bin sha256_mem_halo2 -- --input-size 128
```

`gen_srs` caches structured reference strings under `params/` (override with `PARAMS_DIR`).
That directory is generated and gitignored; the first run at a given `k` creates it.

## Circuit details

`src/circuits.rs` holds `Circuit` impls that pass a message to upstream's `multi_sha256` /
`multi_keccak`. Upstream defines equivalents but only inside `#[cfg(test)]` modules, so they
are not importable and are restated here. All constraints live upstream.

Each input gets the smallest circuit degree `k` that fits it, so proving cost tracks the workload rather than a fixed constant.

The k calcultion is done in `src/bench.rs`, inside `pub fn sha256_dimensions` and `pub fn keecak_dimensions`.

| bytes | SHA256 `k` | Keccak `k` |
| ----: | ---------: | ---------: |
|   128 |         10 |         11 |
|   256 |         10 |         11 |
|   512 |         10 |         12 |
|  1024 |         11 |         13 |
|  2048 |         12 |         14 |

SHA256 has no lookup tables and a fixed column count (~130), so `k` only sets the domain size.
Keccak takes `rows_per_round = 28`, matching upstream's `packed_multi_keccak_simple` test case
`(k: 14, rows_per_round: 28)`; the parameter trades circuit width against height, and for Keccak
`k` is part of `KeccakConfigParams` and also sizes the lookup tables.


## Reported metrics

- `num_constraints` — rows the hash occupies, excluding padding up to `2^k`. Halo2's analogue
  of a gate count: each row is one instance of the circuit's custom gates.
- `preprocessing_size` — serialized proving key. The KZG SRS is universal rather than
  circuit-specific, so it is not counted.
- `proof_size` — transcript length in bytes.

`is_zk` is `false` per the repository's conservative policy: proofs are generated with `OsRng`
and halo2 blinds committed polynomials, but no formal argument covering this exact mode is cited.
