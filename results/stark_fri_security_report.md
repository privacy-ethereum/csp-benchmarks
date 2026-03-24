# STARK / FRI Security Re-Estimation for Default-CI Rust Benchmarks

Date: 2026-03-23

## Scope

This report covers the STARK- or FRI-based Rust benchmarks that are part of the default CI run.

It intentionally excludes the optional crates filtered out by default in `.github/workflows/rust_benchmarks_parallel.yml`: `openvm`, `sp1`, and `nexus`.

`ligetron` is excluded from the main table. The vendored protocol does have a Ligero-style codeword / proximity check: Stage 2 builds `code`, `linear`, and `quadratic` test codewords, Stage 3 reveals sampled openings, and the verifier checks sampled equality plus decoded codeword validity. No local FRI reduction is present: there are no folding rounds, no recursive oracle reduction, and the proof directly includes the full encoded vectors. The implementation therefore appears to be a Ligero-style Reed-Solomon codeword test rather than a FRI-based system for the purposes of this report.

The estimates below are the modified-conjecture re-estimates derived from Crites-Stewart 2025.

## Bottom-Line Table

| System           | Claimed security | Paper-based updated estimate | Delta vs claim | Assessment                                                                                                                               |
| ---------------- | ---------------- | ---------------------------: | -------------: | ---------------------------------------------------------------------------------------------------------------------------------------- |
| `cairo-m`        | 96               |        88.81 bits (floor 88) |          -7.19 | The modified-conjecture estimate is below the advertised 96-bit target                                                                   |
| `miden`          | 128              |        94.33 bits (floor 94) |         -33.67 | The modified-conjecture estimate is below the advertised 128-bit target                                                                  |
| `plonky2`        | 100              |        97.23 bits (floor 97) |          -2.77 | The modified-conjecture estimate is below the advertised 100-bit target                                                                  |
| `provekit`       | 128              |      128.00 bits (floor 128) |           0.00 | The pinned WHIR `ConjectureList` schedule attains 128 bits under the modified mutual-correlated-agreement substitution                   |
| `risc0`          | 96               |        92.41 bits (floor 92) |          -3.59 | The modified-conjecture estimate is below the advertised 96-bit target                                                                   |
| `rookie-numbers` | 96               |        89.70 bits (floor 89) |          -6.30 | The modified-conjecture estimate is below the advertised 96-bit target                                                                   |
| `stark-v`        | 96               |        89.70 bits (floor 89) |          -6.30 | The modified-conjecture estimate is below the advertised 96-bit target                                                                   |

## Method

The quantitative re-estimates in this report are derived from Crites-Stewart 2025. Diamond-Gruen 2025 is used qualitatively to justify discarding the unamended capacity conjecture, but it is not used as an additional numerical penalty. All numerical calculations in this report are reproduced by the companion script [`stark_fri_reestimate.py`](./stark_fri_reestimate.py).

- [Crites-Stewart 2025, "On Reed-Solomon Proximity Gaps Conjectures" (IACR ePrint 2025/2046)](https://eprint.iacr.org/2025/2046)
  - Theorem 7.4.1 gives list-decoding capacity

$$
1 - H_q(\delta).
$$

- Our Conjecture 1 replaces the DEEP-FRI-style threshold

$$
\delta \le 1 - \rho - \eta
$$

by

$$
H_q(\delta) \le 1 - \rho - \eta
$$

for prime fields.

- Our Conjecture 2 replaces the correlated-agreement threshold

$$
\delta \le 1 - \rho - \eta
$$

by

$$
\delta \le 1 - H_q(\delta) - \frac{1}{n} - \eta
$$

for prime fields.

- Our Conjecture 3 replaces WHIR's mutual correlated agreement threshold

$$
0 < \delta < 1 - \rho - \eta
$$

by

$$
0 < H_q(\delta) < 1 - \frac{1}{n} - \rho - \eta
$$

for prime fields.

- [Diamond-Gruen 2025, "On the Distribution of the Distances of Random Words" (IACR ePrint 2025/2010)](https://eprint.iacr.org/2025/2010)
  - Diamond-Gruen 2025 disproves the unamended capacity conjecture with families whose relative rates and relative shortfalls go to 0.
  - The authors' amended Conjecture 5.1 explicitly restricts to practical bounded-rate / bounded-shortfall regimes.
  - Diamond-Gruen 2025 is therefore used as additional support for excluding the old unamended "up to capacity" statements from the security accounting.

### Conjecture mapping

Crites-Stewart 2025's modified conjectures are applied as follows:

- `Our Conjecture 1` from Crites-Stewart 2025
  - Used for the simple FRI-style systems' q-ary-entropy replacement of the old capacity-style per-query term.
  - Used for RISC Zero's toy-model FRI-term replacement.
- `Our Conjecture 2`
  - Used for correlated-agreement-threshold substitutions where the modified bound includes the explicit \(1/n\) term.
- `Our Conjecture 3`
  - Used for ProveKit / WHIR's mutual-correlated-agreement `eta` replacement.

These applications are prime-field applications in the pinned default-CI configurations studied here.

### 1. FRI-style systems

For `cairo-m`, `miden`, `plonky2`, `rookie-numbers`, and `stark-v`, the original code paths all reduce to the usual capacity-style arithmetic:

$$
\mathrm{security}_{\mathrm{old}} = \text{pow\_bits} + \mathrm{queries} \cdot \text{log\_blowup}.
$$

For these systems, the re-estimation keeps the actual PoW term and actual query count fixed, and replaces the original per-query FRI contribution by the q-ary-entropy threshold from the list-decoding-capacity line.

Let:

$$
\begin{aligned}
\rho &= 2^{-\text{log\_blowup}}, \\
H_q(x) &= \frac{x \log(q - 1) - x \log x - (1 - x)\log(1 - x)}{\log q}.
\end{aligned}
$$

Solve:

$$
H_q(\delta^{\ast}) = 1 - \rho.
$$

Then define:

$$
\begin{aligned}
\rho_{\mathrm{eff}} &= 1 - \delta^{\ast}, \\
b_{\mathrm{query}} &= -\log_2(\rho_{\mathrm{eff}}), \\
\mathrm{security}_{\mathrm{new}} &= \text{pow\_bits} + \mathrm{queries} \cdot b_{\mathrm{query}}.
\end{aligned}
$$

For the fields that appear in these default-CI benchmarks, this gives:

| Field / rate         | Old per-query bits | New per-query bits |
| -------------------- | -----------------: | -----------------: |
| M31, rate 1/2        |     1.000000000000 |     0.910063354795 |
| Goldilocks, rate 1/8 |     3.000000000000 |     2.901046720328 |

These are exactly the modest 3% to 9% reductions that Crites-Stewart 2025 says to expect in practical fields.

### 2. ProveKit / WHIR

`provekit` does not use the plain STWO-style arithmetic above. Its benchmark path pins WHIR to `SoundnessType::ConjectureList` with security level 128, starting inverse-rate exponent 1, constant-4 folding, and

$$
\text{pow\_bits} = \text{default\_max\_pow}(\text{num\_variables}, 1).
$$

For the actual benchmark circuits, `results/collected_benchmarks_23330555957.json` shows `provekit` `num_constraints` between `320` and `2,375,009`. In the pinned builder this implies:

$$
\begin{aligned}
m_0 &= \left\lceil \log_2(\text{num\_constraints}) \right\rceil \in [9,22], \\
\text{num\_variables} &= \max\!\left(\left\lceil \log_2(4m_0) \right\rceil + 1,\ 12\right) = 12, \\
\text{pow\_bits} &= 12 + 1 - 3 = 10, \\
\text{protocol\_security\_level} &= 128 - 10 = 118.
\end{aligned}
$$

With constant-4 folding, WHIR's pinned `ConjectureList` scheduler therefore fixes

$$
\begin{aligned}
r &\in \{1,4,7\}, \\
q &= \left\lceil \frac{118}{r} \right\rceil \in \{118,30,17\}, \\
128 - q r &\in \{10,8,9\}.
\end{aligned}
$$

Crites-Stewart 2025 replaces the mutual-correlated-agreement threshold. The analysis therefore keeps the pinned WHIR schedule fixed and replaces only the conjecture-dependent distance threshold.

WHIR computes query counts from the pre-fold rate of the current round and the eta-dependent terms from the post-fold rate that applies after that fold.

For a WHIR stage with local inverse-rate exponent `r`, code length `n`, and old WHIR choice:

$$
\begin{aligned}
\rho &= 2^{-r}, \\
\eta_{\mathrm{old}} &= 2^{-(r+1)}, \\
\delta_{\mathrm{old}} &= 1 - \rho - \eta_{\mathrm{old}}.
\end{aligned}
$$

Crites-Stewart 2025's modified mutual correlated agreement conjecture for prime fields is:

$$
H_q(\delta) < 1 - \frac{1}{n} - \rho - \eta.
$$

To preserve the same boundary distance, solve for the admissible replacement:

$$
\eta_{\mathrm{paper}} = 1 - \frac{1}{n} - \rho - H_q(\delta_{\mathrm{old}}).
$$

The substitution

$$
\log_2(\eta_{\mathrm{paper}})
$$

is then applied to the eta-dependent WHIR terms only:

- `list_size_bits`
- OOD sampling bound
- folding proximity / sumcheck bound
- query-combination bound

The pinned query schedule is **not** changed, because WHIR's query arithmetic \(q \cdot r\) is not the part invalidated by Crites-Stewart 2025.

#### ProveKit calculation

Start-stage checks:

| Check          | Code length `n` | Paper-updated bits |
| -------------- | --------------: | -----------------: |
| Commitment OOD |            8192 |         212.961412 |
| Starting fold  |            8192 |         237.980706 |

Round and final query checks:

| Stage   | Old query rate | Post-fold rate | Code length `n` |    `delta_old` |    `eta_paper` | Query bits | Combination bits | Added PoW | Stage total |
| ------- | -------------: | -------------: | --------------: | -------------: | -------------: | ---------: | ---------------: | --------: | ----------: |
| Round 1 |              1 |              4 |            4096 | 0.906250000000 | 0.029235865901 | 118.000000 |       229.009065 | 10.000000 |  128.000000 |
| Round 2 |              4 |              7 |            2048 | 0.988281250000 | 0.003055253639 | 120.000000 |       228.691312 |  8.000000 |  128.000000 |
| Final   |              7 |              7 |     final phase |            n/a |            n/a | 119.000000 |              n/a |  9.000000 |  128.000000 |

The folding checks in those two rounds also stay comfortably above target:

- Round 1 folding: `235.903883`
- Round 2 folding: `233.645508`

The overall bottleneck is therefore `128.000000` bits.

### 3. RISC Zero

The metadata field in `risc0/src/lib.rs` reports `96` bits. The pinned upstream soundness commentary states that the verifier target uses the Toy Problem Conjecture, so the relevant paper-based comparison is the updated `toy_model_security()` path.

Local constants for the pinned configuration:

$$
\begin{aligned}
q &= 15 \cdot 2^{27} + 1 = 2013265921, \\
\mathrm{queries} &= 50, \\
\rho &= \frac{1}{4}, \\
\eta &= 0.05.
\end{aligned}
$$

$$
\begin{aligned}
\text{fri\_fold} &= 16, \\
\text{segment\_size} &= 2^{20}, \\
w_{\mathrm{accum}} &= 103, \\
w_{\mathrm{code}} &= 1, \\
w_{\mathrm{data}} &= 211.
\end{aligned}
$$

$$
\begin{aligned}
n_{\text{trace\_polys}} &= 315, \\
\text{biggest\_combo} &= 6.
\end{aligned}
$$

#### 3a. Toy-model update

The local toy-model path uses:

$$
\text{fri\_error}_{\mathrm{old}} = \rho^{\mathrm{queries}}.
$$

The paper-compatible replacement is the same q-ary-entropy adjustment used for the simple FRI systems:

$$
\begin{aligned}
H_q(\delta^{\ast}) &= 1 - \rho, \\
\rho_{\mathrm{eff}} &= 1 - \delta^{\ast}, \\
\text{fri\_error}_{\mathrm{new}} &= \rho_{\mathrm{eff}}^{\mathrm{queries}}.
\end{aligned}
$$

For BabyBear:

$$
\begin{aligned}
\delta^{\ast} &= 0.722429470819, \\
\rho_{\mathrm{eff}} &= 0.277570529181.
\end{aligned}
$$

Replacing only the FRI term and leaving the PLONK/PLOOKUP and constraint terms unchanged gives:

| Path                   | Upstream local figure | Paper-based updated figure |
| ---------------------- | --------------------: | -------------------------: |
| `toy_model_security()` |             97.141981 |                  92.406234 |

The primary toy-model estimate is `92.41` bits.

## Per-System Calculations

### FRI-style systems

- `cairo-m`

$$
\begin{aligned}
\text{pow\_bits} &= 16, \\
\mathrm{queries} &= 80, \\
\text{log\_blowup} &= 1.
\end{aligned}
$$

$$
\mathrm{security}_{\mathrm{new}} = 16 + 80 \cdot 0.910063354795 = 88.805068.
$$

- `miden`

$$
\begin{aligned}
\text{pow\_bits} &= 16, \\
\mathrm{queries} &= 27, \\
\text{log\_blowup} &= 3.
\end{aligned}
$$

$$
\mathrm{security}_{\mathrm{new}} = 16 + 27 \cdot 2.901046720328 = 94.328261.
$$

- `plonky2`

$$
\begin{aligned}
\text{pow\_bits} &= 16, \\
\mathrm{queries} &= 28, \\
\text{log\_blowup} &= 3.
\end{aligned}
$$

$$
\mathrm{security}_{\mathrm{new}} = 16 + 28 \cdot 2.901046720328 = 97.229308.
$$

- `rookie-numbers`

$$
\begin{aligned}
\text{pow\_bits} &= 26, \\
\mathrm{queries} &= 70, \\
\text{log\_blowup} &= 1.
\end{aligned}
$$

$$
\mathrm{security}_{\mathrm{new}} = 26 + 70 \cdot 0.910063354795 = 89.704435.
$$

- `stark-v`

$$
\begin{aligned}
\text{pow\_bits} &= 26, \\
\mathrm{queries} &= 70, \\
\text{log\_blowup} &= 1.
\end{aligned}
$$

$$
\mathrm{security}_{\mathrm{new}} = 26 + 70 \cdot 0.910063354795 = 89.704435.
$$

### System-specific notes

- `cairo-m`
  - The pinned STWO-style arithmetic yields `88.81` bits when the capacity-style query term is replaced by the Crites-Stewart 2025 q-ary entropy threshold.
- `miden`
  - The metadata claim is `128`.
  - Under the Crites-Stewart 2025 replacement, the benchmark configuration evaluates to `94.33` bits.
- `plonky2`
  - The explicit "Conjectured FRI security" path evaluates to `97.23` bits under the modified conjecture.
- `provekit`
  - The pinned WHIR schedule evaluates to `128` bits under the Crites-Stewart 2025 substitution.
- `risc0`
  - The Crites-Stewart 2025-based update of the toy-model path is `92.41`.
- `rookie-numbers` and `stark-v`
  - These share the same STWO-style arithmetic and therefore the same adjusted value: `89.70`.

## Inputs Used

- Benchmark metadata and measurements:
  - `results/collected_benchmarks_23330555957.json`
- Report reproduction script:
  - [`results/stark_fri_reestimate.py`](./stark_fri_reestimate.py)
- Benchmark-local code:
  - `cairo-m/src/lib.rs`
  - `miden/src/lib.rs`
  - `plonky2/src/lib.rs`
  - `provekit/src/lib.rs`
  - `risc0/src/lib.rs`
  - `rookie-numbers/src/lib.rs`
  - `stark-v/src/lib.rs`
- Locked upstream code paths already inspected for parameters:
  - Cairo/STWO `prover_config.rs`, `core/fri.rs`, `core/pcs/mod.rs`
  - Miden / `p3-miden-fri`
  - Plonky2 FRI config
  - WHIR `parameters.rs` and `whir/parameters.rs`
  - RISC Zero `soundness.rs`, circuit tap metadata, and test fixture
- Supplied papers:
  - [Crites-Stewart 2025, "On Reed-Solomon Proximity Gaps Conjectures" (IACR ePrint 2025/2046)](https://eprint.iacr.org/2025/2046)
  - [Diamond-Gruen 2025, "On the Distribution of the Distances of Random Words" (IACR ePrint 2025/2010)](https://eprint.iacr.org/2025/2010)

## Checked But Excluded: Ligetron

`ligetron` uses an encoded-codeword test internally and was therefore examined separately.

What the local code shows:

- The benchmark metadata still classifies it as `Ligero`, not `FRI`, in `ligetron/bench_props.json`.
- The protocol parameters are Ligero-style: `default_row_size = 8192`, `default_encoding_size = 4 * default_row_size`, and `sample_size = 192`.
- Stage 2 accumulates three test codewords, `code`, `linear`, and `quadratic`, and Stage 3 serializes sampled openings.
- The proof also serializes the full encoded vectors `encoded_code_limbs`, `encoded_linear_limbs`, and `encoded_quad_limbs`.
- The verifier decodes those full vectors and checks:
  - `valid_code`: coefficients above the degree bound are zero
  - `valid_linear`: the linear consistency sum is zero
  - `valid_quad`: the quadratic consistency polynomial is zero
  - sampled equality between the prover's encoded vectors and the verifier's recomputed sampled positions

Why it is still excluded from the main table:

- This is a Ligero-style RS codeword / proximity check rather than a FRI reduction in the local implementation.
- The same capacity-style or proximity-gap conjectural formula used to re-price `cairo-m`, `miden`, `plonky2`, `provekit`, `risc0`, `rookie-numbers`, and `stark-v` does not appear in the local `ligetron` code path.
- The benchmark's `128` bits appears as static metadata in `ligetron/bench_props.json`; no local derivation tying it to the two supplied conjectures was found.

Accordingly, `ligetron` is excluded from the main STARK / FRI re-estimation table.

## Conclusion

Under the Crites-Stewart 2025-based modified-conjecture replacements above:

- `provekit` attains its claimed `128` bits.
- `cairo-m`, `miden`, `plonky2`, `risc0`, `rookie-numbers`, and `stark-v` all fall below their currently advertised benchmark targets.

The largest metadata-level gap is `miden`, which goes from `128` to `94.33` under the Crites-Stewart 2025-based replacement. For `risc0`, the metadata claim is `96` and the paper-based updated estimate is `92.41` bits. `plonky2` has the smallest gap, moving from `100` to `97.23`.
