# Benchmark Security-Bit Estimates

Date: 2026-06-04

## Scope

This report records the security-bit values used in the current benchmark result metadata. It only reports the soundness estimate for the security mode actually configured by each benchmarked implementation. It does not try other calculators or regimes unless the implementation is configured to use them.

The current result artifact is:

- `results/collected_benchmarks_26644954256.json`

## Reported Metadata

| System             | Reported bits | Previous target / metadata            | Basis                                                                   |
| ------------------ | ------------: | ------------------------------------- | ----------------------------------------------------------------------- |
| `binius64`         |            96 | 96                                    | Not re-estimated here; no applicable configured-mode estimator was used |
| `cairo-m`          |            88 | 96                                    | Crites-Stewart floor for configured `REGULAR_96_BITS` FRI               |
| `circom`           |           100 | 128                                   | BN254 Groth16 effective curve-security cap                              |
| `expander`         |           128 | 128                                   | Not re-estimated here; no applicable configured-mode estimator was used |
| `jolt`             |           100 | 128                                   | Bn254 Dory effective curve-security cap                                 |
| `miden`            |            94 | 128 metadata / 96 pinned proof target | Crites-Stewart floor for configured Miden FRI                           |
| `plonky2`          |            97 | 100                                   | Crites-Stewart floor for configured Plonky2 recursion FRI               |
| `provekit`         |           128 | 128                                   | Configured Johnson-bound WHIR                                           |
| `provekit-groth16` |           100 | 128                                   | BN254 Groth16 effective curve-security cap                              |
| `risc0`            |            96 | 96 RISC-V prover target               | Configured RISC Zero 3.0 security model                                 |
| `rookie-numbers`   |            89 | 96                                    | Crites-Stewart floor for configured `secure_pcs_config()`               |
| `spartan2`         |           128 | 128                                   | Not re-estimated here; no applicable configured-mode estimator was used |
| `stark-v`          |            94 | 94 upstream batching cap              | Configured upstream UDR/soundcalc batching cap for trace <= 2^20        |
| `barretenberg`     |           100 | 128                                   | BN254 UltraHonk / KZG effective curve-security cap                      |
| `ligetron`         |           128 | 128                                   | Not re-estimated here; no applicable configured-mode estimator was used |

## Reproducible Estimates

These values are reproduced by:

```bash
python3 results/stark_fri_reestimate.py
```

| System           | Metadata bits |   Estimate | Floor | Calculation                                                        |
| ---------------- | ------------: | ---------: | ----: | ------------------------------------------------------------------ |
| `cairo-m`        |            88 |  88.805068 |    88 | `16 + 80 * 0.910063354795`                                         |
| `miden`          |            94 |  94.328261 |    94 | `16 + 27 * 2.901046720328`                                         |
| `plonky2`        |            97 |  97.229308 |    97 | `16 + 28 * 2.901046720328`                                         |
| `rookie-numbers` |            89 |  89.704435 |    89 | `26 + 70 * 0.910063354795`                                         |
| `provekit`       |           128 | 128.000000 |   128 | configured Johnson-bound WHIR                                      |
| `stark-v`        |            94 |  94.963824 |    94 | configured upstream UDR/soundcalc; batching bottleneck             |
| `risc0`          |            96 |  97.141981 |    97 | configured RISC Zero toy-model reproduction; metadata target is 96 |

For simple FRI-style systems, the previous benchmark arithmetic was:

$$
\mathrm{security}_{\mathrm{old}} =
\mathrm{pow}_{\mathrm{bits}} + \mathrm{queries} \cdot \mathrm{log}_{\mathrm{blowup}}.
$$

The Crites-Stewart re-estimation is used only for benchmarked systems whose configured security model is this simple-FRI path. It keeps the configured PoW term and query count fixed, then replaces the per-query capacity-style term with the q-ary entropy threshold:

$$
H_q(\delta^\ast) = 1 - \rho,\quad
\rho_{\mathrm{eff}} = 1 - \delta^\ast,\quad
b_{\mathrm{query}} = -\log_2(\rho_{\mathrm{eff}}).
$$

For the fields used by the benchmarked simple-FRI systems:

| Field / rate         | Old per-query bits | New per-query bits |
| -------------------- | -----------------: | -----------------: |
| M31, rate 1/2        |     1.000000000000 |     0.910063354795 |
| Goldilocks, rate 1/8 |     3.000000000000 |     2.901046720328 |

## RISC Zero

The benchmark uses `ere-risc0`, whose default segment limit is `ERE_RISC0_SEGMENT_PO2=20` unless overridden. No benchmark override is present.

The configured RISC Zero security model reports the RISC-V prover target as 96 bits. The local `risc0-zkp` toy-model calculation, using the live `rv32im` tapset at the default `2^20` segment, reproduces `97.141981` bits. In that configured model, the argument term is the bottleneck; the FRI query term `rho^50` contributes 100 bits and is not limiting. The reported metadata therefore remains `96`.

## Stark-V

Stark-V upstream `secure_pcs_config()` is configured for UDR/soundcalc accounting. The current upstream configuration is `FriConfig::new(0, 1, 193, 4)` with `pow_bits = 16`, over `M31^4`, with no lifting.

For trace sizes <= `2^20`, the configured soundcalc/UDR phase values are:

| Phase         |                 Bits |
| ------------- | -------------------: |
| batching      |            94.963824 |
| query phase   |            96.102237 |
| commit rounds | 105.093065 or higher |

The bottleneck is batching, which is not strengthened by query-phase PoW, so the reported floor is `94`. This matches the 94 bits specified in metadata.

## ProveKit / WHIR

`provekit` uses WHIR rather than the simple FRI arithmetic above. The benchmark path pins ProveKit to a WHIR configuration tuned for 128-bit security under the Johnson bound:

| Parameter                 |   Value |
| ------------------------- | ------: |
| `unique_decoding`         | `false` |
| `security_level`          |     128 |
| `pow_bits`                |      10 |
| `protocol_security_level` |     118 |
| `starting_log_inv_rate`   |       2 |
| `initial_folding_factor`  |       3 |
| `folding_factor`          |       3 |
| `batch_size`              |       1 |
| minimum witness variables |      13 |

For the actual benchmark circuits in `results/collected_benchmarks_26644954256.json`, `provekit` has `num_constraints` between `231` and `1,575,606`, so `ceil(log2(num_constraints))` is in `[8, 21]`. The result artifact does not record exact witness counts, so the script mirrors the pinned ProveKit/WHIR security calculator over witness-variable sizes `m = 13..24`, covering the constraint exponent range plus margin.

Across that checked benchmark range, the worst case is still `128.000000` bits. The worst checked configuration has `m = 13`, blinding variables `ell = 12`, `q_delta_1 = 127`, `q_delta_2 = 138`, and requested PoW below the configured 10-bit budget. The reported metadata therefore remains `128`.

## BN254 / Bn254 Systems

`circom`, `jolt`, `provekit-groth16`, and `barretenberg` use BN254 / Bn254 pairing-based machinery in the benchmarked configuration:

- `circom`: Groth16 over Bn254.
- `jolt`: Dory over `Bn254Curve`.
- `provekit-groth16`: Groth16 over Bn254.
- `barretenberg`: UltraHonk with KZG over BN254.

Although these systems previously used 128-bit metadata, BN254-family pairing-friendly curves are treated as about 100-bit-security legacy curves after exTNFS. The metadata therefore reports `100` bits for these systems.

Reference:

- [CFRG Pairing-Friendly Curves draft](https://cfrg.github.io/draft-irtf-cfrg-pairing-friendly-curves/draft-irtf-cfrg-pairing-friendly-curves.html)

## Not Re-Estimated Here

The following systems keep their existing metadata because no applicable configured-mode estimator was used in this report:

- `binius64`: existing metadata is `96`.
- `expander`: existing metadata is `128`.
- `spartan2`: existing metadata is `128`.
- `ligetron`: existing metadata is `128`; the local implementation is Ligero-style rather than FRI-style for the purposes of the Crites-Stewart estimator.

These entries should be revisited if an appropriate system-specific estimator is added.

## Inputs Used

- Benchmark metadata and measurements:
  - `results/collected_benchmarks_26644954256.json`
- Report reproduction script:
  - [`results/stark_fri_reestimate.py`](./stark_fri_reestimate.py)
- [Soundcalc](https://github.com/ethereum/soundcalc/commit/809896fb8d3aba4fd8f657c781601e3ef2b968dd) checkout:
  - configured Stark-V UDR direct API cross-check
- External references checked:
  - [Cairo-M `REGULAR_96_BITS`](https://github.com/kkrt-labs/cairo-m/blob/79c1b1e83f7b959babe3c991eca0902954a31385/crates/prover/src/prover_config.rs)
  - [csp-benchmarks PR #270](https://github.com/privacy-ethereum/csp-benchmarks/pull/270)
  - [Stark-V `secure_pcs_config()`](https://github.com/ClementWalter/stark-v/blob/0bf633bc5a87b14d2e2ad4d8f30be2701849e0c1/crates/sdk/src/lib.rs)
  - [ProveKit pinned WHIR config](https://github.com/worldfnd/ProveKit/blob/cc391c8cc72766cc47f8243402e7f51e16c6d7cd/provekit/r1cs-compiler/src/whir_r1cs.rs)
  - [RISC Zero security model](https://dev.risczero.com/api/security-model)
  - [RISC Zero soundness notebook](https://github.com/risc0/risc0/blob/main/risc0/zkp/src/docs/soundness.ipynb)

## Conclusion

The benchmark metadata now reports the soundness of the systems as currently configured:

- Crites-Stewart floors are used only where that estimator matches the benchmarked simple-FRI path: `cairo-m`, `miden`, `plonky2`, and `rookie-numbers`.
- `provekit` remains `128` under its configured Johnson-bound WHIR parameters.
- `stark-v` reports `94`, matching its configured upstream UDR/soundcalc batching cap.
- `risc0` reports `96`, matching its configured RISC Zero 3.0 security model.
- BN254 / Bn254 pairing-based systems are reported as `100`.
- Systems without an applicable configured-mode estimator in this report are explicitly marked as not re-estimated here.
