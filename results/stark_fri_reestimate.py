#!/usr/bin/env python3
"""Re-estimate default-CI STARK / FRI benchmark security after the 2025 papers.

This script keeps the calculations used in
results/stark_fri_security_report.md in one reproducible place.

The outputs are estimates that match the benchmarked systems' configured security models.

Reference map for the paper-derived substitutions:

- Crites-Stewart 2025, "On Reed-Solomon Proximity Gaps Conjectures"
  (IACR ePrint 2025/2046, https://eprint.iacr.org/2025/2046)
  - Theorem 7.4.1 and the displayed definition of H_q immediately below it:
    q-ary entropy / list-decoding capacity. In the pdftotext extraction used
    locally, this is the block at lines 275-282.
  - Introduction paragraph:
    the "loss of 1 / log2 q in the error rate" summary is the paragraph at
    lines 83-86.

Several helper formulas below are intentionally mirrored from pinned local code
paths rather than from either paper. Those blocks are labeled explicitly so the
paper-driven substitutions can be distinguished from "reproduce upstream code"
machinery.
"""

from __future__ import annotations

import argparse
import json
import math
from pathlib import Path

M31 = 2**31 - 1
GOLDILOCKS = 2**64 - 2**32 + 1
BABY_BEAR = 15 * 2**27 + 1
BN254_SCALAR_MODULUS = int(
    "30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000001", 16
)
LOG2_10 = math.log2(10.0)


def h_q(delta: float, q: int) -> float:
    # Crites-Stewart 2025, "On Reed-Solomon Proximity Gaps Conjectures",
    # Theorem 7.4.1 and the displayed definition of H_q immediately below it;
    # pdftotext lines 275-282.
    if delta <= 0.0:
        return 0.0
    if delta >= 1.0 - 1.0 / q:
        return 1.0
    return (
        delta * math.log(q - 1)
        - delta * math.log(delta)
        - (1.0 - delta) * math.log(1.0 - delta)
    ) / math.log(q)


def invert_h_q(target: float, q: int, iterations: int = 300) -> float:
    # Numerical inversion of the q-ary entropy line from Crites-Stewart 2025,
    # Theorem 7.4.1; pdftotext lines 275-282.
    # This inversion step is our implementation device; the paper gives H_q
    # explicitly but not a closed-form inverse.
    lo = 0.0
    hi = 1.0 - 1.0 / q
    for _ in range(iterations):
        mid = (lo + hi) / 2.0
        if h_q(mid, q) < target:
            lo = mid
        else:
            hi = mid
    return (lo + hi) / 2.0


def estimate_fri_bits(q: int, log_blowup: int, queries: int, pow_bits: int) -> dict:
    rho = 2.0 ** (-log_blowup)
    # Paper-driven substitution:
    # - Crites-Stewart 2025, Theorem 7.4.1 gives list-decoding capacity
    #   1 - H_q(delta); pdftotext lines 275-282.
    # - Crites-Stewart 2025, Our Conjecture 1 replaces the old prime-field
    #   DEEP-FRI-style threshold by an H_q threshold; pdftotext lines 360-366.
    #
    # The script's boundary solve H_q(delta*) = 1 - rho is therefore an
    # inference from Our Conjecture 1 plus the report's "hold the rate fixed
    # and re-price the per-query term" methodology; this exact solve is not
    # printed verbatim in the paper.
    delta_star = invert_h_q(1.0 - rho, q)
    rho_eff = 1.0 - delta_star
    bits_per_query = -math.log2(rho_eff)
    # This "-log2(success probability)" conversion is standard soundness-bit
    # accounting, not a paper-specific formula.
    total_bits = pow_bits + queries * bits_per_query
    return {
        "rho": rho,
        "delta_star": delta_star,
        "rho_eff": rho_eff,
        "bits_per_query": bits_per_query,
        "total_bits": total_bits,
        "floor_bits": math.floor(total_bits),
    }


def error_bits(error: float) -> float:
    return -math.log2(error)


def estimate_stark_v() -> dict:
    # Mirrored from upstream stark-v secure_pcs_config():
    # FriConfig::new(0, 1, 193, 4), pow_bits=16, M31^4, no lifting.
    # The analysis comment in that upstream config targets UDR and says the
    # batching phase, which is not strengthened by query-phase PoW, caps
    # trace sizes <= 2^20 at 94 bits. This reproduces the same bottleneck.
    field_size = M31**4
    rate = 0.5
    trace_length = 1 << 20
    batch_size = 1051
    queries = 193
    pow_bits = 16
    folding_factors = [16, 16, 16, 16, 16, 2]
    proximity = (1.0 - rate) / 2.0

    def powers_batching_error(dimension: float, num_functions: int) -> float:
        codeword_size = dimension / rate
        return ((proximity * codeword_size + 1.0) / field_size) * (
            num_functions - 1
        )

    batching_bits = error_bits(powers_batching_error(trace_length, batch_size))

    commit_bits = []
    dimension = float(trace_length)
    for folding_factor in folding_factors:
        dimension /= folding_factor
        commit_bits.append(
            error_bits(powers_batching_error(dimension, folding_factor))
        )

    query_bits = error_bits(((1.0 - proximity) ** queries) / (2**pow_bits))
    total_bits = min([batching_bits, query_bits, *commit_bits])

    return {
        "field_q": field_size,
        "rate": rate,
        "trace_length": trace_length,
        "batch_size": batch_size,
        "queries": queries,
        "pow_bits": pow_bits,
        "proximity": proximity,
        "batching_bits": batching_bits,
        "commit_bits": commit_bits,
        "query_bits": query_bits,
        "total_bits": total_bits,
        "floor_bits": math.floor(total_bits),
    }


def ceil_log2(n: int) -> int:
    if n <= 1:
        return 0
    return (n - 1).bit_length()


def load_measurements(path: Path) -> list[dict]:
    payload = json.loads(path.read_text())
    return payload["measurements"]


def field_size_bits(q: int) -> float:
    # Mirrors whir::algebra::fields::FieldWithSize::field_size_bits for prime
    # fields: log2(modulus) times extension degree.
    return math.log2(q)


def pow_difficulty_bits(requested_bits: float) -> float:
    # Mirrors whir::protocols::proof_of_work::{threshold,difficulty}. WHIR
    # stores a u64 threshold, so fractional difficulties are quantized.
    if not 0.0 <= requested_bits <= 60.0:
        raise ValueError(f"PoW difficulty out of WHIR range: {requested_bits}")
    threshold = math.ceil(2.0 ** (64.0 - requested_bits))
    if threshold >= 2**64 - 1:
        threshold = 2**64 - 1
    return 64.0 - math.log2(threshold)


def whir_num_in_domain_queries(
    *, unique_decoding: bool, security_target: float, rate: float
) -> int:
    # Mirrors whir::protocols::irs_commit::num_in_domain_queries.
    if unique_decoding:
        per_sample = (1.0 + rate) / 2.0
    else:
        per_sample = math.sqrt(rate) + math.sqrt(rate) / 20.0
    return math.ceil(security_target / -math.log2(per_sample))


class WhirIrsEstimate:
    """Small mirror of whir::protocols::irs_commit::Config security math."""

    def __init__(
        self,
        *,
        field_bits: float,
        security_target: float,
        unique_decoding: bool,
        num_vectors: int,
        vector_size: int,
        interleaving_depth: int,
        rate: float,
    ) -> None:
        if vector_size % interleaving_depth != 0:
            raise ValueError("WHIR vector_size must be divisible by interleaving_depth")
        self.field_bits = field_bits
        self.security_target = security_target
        self.unique_decoding = unique_decoding
        self.num_vectors = num_vectors
        self.vector_size = vector_size
        self.interleaving_depth = interleaving_depth
        self.message_length = vector_size // interleaving_depth
        self.codeword_length = math.ceil(self.message_length / rate)
        self.rate = self.message_length / self.codeword_length
        self.johnson_slack = 0.0 if unique_decoding else math.sqrt(self.rate) / 20.0

        if unique_decoding:
            self.out_domain_samples = 0
        else:
            list_size = self.list_size()
            l_choose_2 = list_size * (list_size - 1.0) / 2.0
            log_per_sample = self.field_bits - math.log2(vector_size - 1)
            if log_per_sample <= 0.0:
                raise ValueError("WHIR OOD sample calculation exceeds field capacity")
            self.out_domain_samples = int(
                max(
                    1.0,
                    math.ceil((security_target + math.log2(l_choose_2)) / log_per_sample),
                )
            )

        self.in_domain_samples = whir_num_in_domain_queries(
            unique_decoding=unique_decoding,
            security_target=security_target,
            rate=self.rate,
        )

    def list_size(self) -> float:
        if self.unique_decoding:
            return 1.0
        return 1.0 / (2.0 * self.johnson_slack * math.sqrt(self.rate))

    def rbr_ood_sample(self) -> float:
        list_size = self.list_size()
        l_choose_2 = list_size * (list_size - 1.0) / 2.0
        log_per_sample = math.log2(self.vector_size - 1) - self.field_bits
        return -math.log2(l_choose_2) - self.out_domain_samples * log_per_sample

    def rbr_queries(self) -> float:
        if self.unique_decoding:
            per_sample = (1.0 + self.rate) / 2.0
        else:
            per_sample = math.sqrt(self.rate) + self.johnson_slack
        return self.in_domain_samples * -math.log2(per_sample)

    def rbr_soundness_fold_prox_gaps(self) -> float:
        log_inv_rate = -math.log2(self.rate)
        log_k = math.log2(self.message_length)
        if self.unique_decoding:
            error = log_k + log_inv_rate
        else:
            log_eta = math.log2(self.johnson_slack)
            min_eta = -(0.5 * log_inv_rate + LOG2_10 + 1.0) - 1e-6
            if log_eta < min_eta:
                raise ValueError("WHIR Johnson slack is below the minimum bound")
            error = 7.0 * LOG2_10 + 3.5 * log_inv_rate + 2.0 * log_k
        return self.field_bits - error


class WhirConfigEstimate:
    """Small mirror of whir::protocols::whir::Config security_level()."""

    def __init__(
        self,
        *,
        field_bits: float,
        size: int,
        unique_decoding: bool,
        security_level: int,
        pow_bits: int,
        initial_folding_factor: int,
        folding_factor: int,
        starting_log_inv_rate: int,
        batch_size: int,
    ) -> None:
        self.field_bits = field_bits
        self.size = size
        self.unique_decoding = unique_decoding
        self.security_level = float(security_level)
        self.protocol_security_level = float(security_level - pow_bits)
        self.initial_folding_factor = initial_folding_factor
        self.folding_factor = folding_factor
        self.starting_log_inv_rate = starting_log_inv_rate
        self.batch_size = batch_size

        self.initial_committer = WhirIrsEstimate(
            field_bits=field_bits,
            security_target=self.protocol_security_level,
            unique_decoding=unique_decoding,
            num_vectors=batch_size,
            vector_size=size,
            interleaving_depth=1 << initial_folding_factor,
            rate=2.0 ** (-starting_log_inv_rate),
        )

        initial_prox = self.initial_committer.rbr_soundness_fold_prox_gaps()
        initial_sumcheck = field_bits - math.log2(self.initial_committer.list_size()) - 1.0
        self.starting_folding_pow_bits = max(
            self.security_level - min(initial_prox, initial_sumcheck), 0.0
        )
        self.initial_skip_pow_bits = max(
            self.security_level
            - (initial_prox + math.log2(initial_folding_factor)),
            0.0,
        )

        self.rounds: list[dict] = []
        num_variables = int(math.log2(size))
        log_inv_rate = starting_log_inv_rate
        in_domain_samples = self.initial_committer.in_domain_samples
        query_error = self.initial_committer.rbr_queries()
        num_variables -= initial_folding_factor
        round_index = 0
        while num_variables >= folding_factor:
            round_folding_factor = (
                initial_folding_factor if round_index == 0 else folding_factor
            )
            next_rate = log_inv_rate + (round_folding_factor - 1)
            irs = WhirIrsEstimate(
                field_bits=field_bits,
                security_target=self.protocol_security_level,
                unique_decoding=unique_decoding,
                num_vectors=1,
                vector_size=1 << num_variables,
                interleaving_depth=1 << folding_factor,
                rate=2.0 ** (-next_rate),
            )
            combination_error = field_bits - (
                math.log2(irs.out_domain_samples + in_domain_samples)
                + math.log2(irs.list_size())
                + 1.0
            )
            pow_added = max(
                self.security_level - min(query_error, combination_error), 0.0
            )
            folding_pow_added = max(
                self.security_level
                - min(
                    irs.rbr_soundness_fold_prox_gaps(),
                    field_bits - (math.log2(irs.list_size()) + 1.0),
                ),
                0.0,
            )
            self.rounds.append(
                {
                    "num_variables": num_variables,
                    "next_log_inv_rate": next_rate,
                    "irs": irs,
                    "pow_bits": pow_added,
                    "folding_pow_bits": folding_pow_added,
                    "combination_error": combination_error,
                    "query_error": query_error,
                }
            )
            round_index += 1
            num_variables -= folding_factor
            log_inv_rate = next_rate
            in_domain_samples = irs.in_domain_samples
            query_error = irs.rbr_queries()

        self.final_sumcheck_num_rounds = num_variables
        rbr_error = (
            self.rounds[-1]["irs"].rbr_queries()
            if self.rounds
            else self.initial_committer.rbr_queries()
        )
        self.final_pow_bits = max(self.security_level - rbr_error, 0.0)
        self.final_folding_pow_bits = max(
            self.security_level - field_bits + 1.0, 0.0
        )

    def security_terms(self, *, num_vectors: int, num_linear_forms: int) -> list[tuple[str, float]]:
        terms: list[tuple[str, float]] = []
        if num_vectors > 1:
            terms.append(("vector RLC", self.field_bits - math.log2(num_vectors - 1)))
        if num_linear_forms > 1:
            terms.append(
                ("linear-form RLC", self.field_bits - math.log2(num_linear_forms - 1))
            )

        has_initial_constraints = (
            num_linear_forms > 0 or self.initial_committer.out_domain_samples > 0
        )
        if not self.initial_committer.unique_decoding:
            terms.append(("initial OOD", self.initial_committer.rbr_ood_sample()))

        initial_prox = self.initial_committer.rbr_soundness_fold_prox_gaps()
        if has_initial_constraints:
            initial_sumcheck = self.field_bits - (
                math.log2(self.initial_committer.list_size()) + 1.0
            )
            terms.append(
                (
                    "initial fold",
                    min(initial_prox, initial_sumcheck)
                    + pow_difficulty_bits(self.starting_folding_pow_bits),
                )
            )
        else:
            terms.append(
                (
                    "initial skip fold",
                    initial_prox
                    + math.log2(self.initial_folding_factor)
                    + pow_difficulty_bits(self.initial_skip_pow_bits),
                )
            )

        rbr_queries = self.initial_committer.rbr_queries()
        old_in_domain_samples = self.initial_committer.in_domain_samples
        for index, round_info in enumerate(self.rounds):
            irs: WhirIrsEstimate = round_info["irs"]
            if not irs.unique_decoding:
                terms.append((f"round {index} OOD", irs.rbr_ood_sample()))

            combination_error = self.field_bits - (
                math.log2(irs.out_domain_samples + old_in_domain_samples)
                + math.log2(irs.list_size())
                + 1.0
            )
            terms.append(
                (
                    f"round {index} query",
                    min(rbr_queries, combination_error)
                    + pow_difficulty_bits(round_info["pow_bits"]),
                )
            )
            terms.append(
                (
                    f"round {index} fold",
                    min(
                        irs.rbr_soundness_fold_prox_gaps(),
                        self.field_bits - (math.log2(irs.list_size()) + 1.0),
                    )
                    + pow_difficulty_bits(round_info["folding_pow_bits"]),
                )
            )
            old_in_domain_samples = irs.in_domain_samples
            rbr_queries = irs.rbr_queries()

        terms.append(
            ("final query", rbr_queries + pow_difficulty_bits(self.final_pow_bits))
        )
        if self.final_sumcheck_num_rounds > 0:
            terms.append(
                (
                    "final combination",
                    self.field_bits
                    - 1.0
                    + pow_difficulty_bits(self.final_folding_pow_bits),
                )
            )
        return terms

    def security_level_bits(self, *, num_vectors: int, num_linear_forms: int) -> float:
        terms = self.security_terms(
            num_vectors=num_vectors, num_linear_forms=num_linear_forms
        )
        return min((bits for _, bits in terms), default=0.0)

    def max_requested_pow_bits(self) -> float:
        candidates = [
            self.starting_folding_pow_bits,
            self.initial_skip_pow_bits,
            self.final_pow_bits,
            self.final_folding_pow_bits,
        ]
        for round_info in self.rounds:
            candidates.append(round_info["pow_bits"])
            candidates.append(round_info["folding_pow_bits"])
        return max(candidates)


def provekit_whir_security_for_nv(num_variables: int) -> dict:
    # Mirrored from pinned ProveKit r1cs-compiler/src/whir_r1cs.rs:
    # ProtocolParameters { unique_decoding=false, security_level=128,
    # pow_bits=10, initial_folding_factor=3, folding_factor=3,
    # starting_log_inv_rate=2, batch_size=1 }.
    field_bits = field_size_bits(BN254_SCALAR_MODULUS)
    params = {
        "field_bits": field_bits,
        "unique_decoding": False,
        "security_level": 128,
        "pow_bits": 10,
        "initial_folding_factor": 3,
        "folding_factor": 3,
        "starting_log_inv_rate": 2,
    }
    blinded = WhirConfigEstimate(size=1 << num_variables, batch_size=1, **params)

    q_delta_1 = whir_num_in_domain_queries(
        unique_decoding=False,
        security_target=params["security_level"] - params["pow_bits"],
        rate=2.0 ** (-params["starting_log_inv_rate"]),
    )
    q_delta_2 = whir_num_in_domain_queries(
        unique_decoding=False,
        security_target=params["security_level"],
        rate=2.0 ** (-params["starting_log_inv_rate"]),
    )
    k1 = 1 << params["initial_folding_factor"]
    k2 = 1 << params["initial_folding_factor"]
    query_upper_bound = (
        k1 * q_delta_1
        + k2 * q_delta_2
        + q_delta_1
        + q_delta_2
        + 4 * num_variables
    )
    num_blinding_variables = query_upper_bound.bit_length()
    if num_blinding_variables >= num_variables:
        raise ValueError(
            "ProveKit WHIR blinding variables must be fewer than witness variables"
        )

    blinding = WhirConfigEstimate(
        size=1 << num_blinding_variables,
        batch_size=num_variables + 1,
        **params,
    )
    blinded_terms = blinded.security_terms(
        num_vectors=blinded.initial_committer.num_vectors, num_linear_forms=1
    )
    blinding_terms = blinding.security_terms(
        num_vectors=blinding.initial_committer.num_vectors, num_linear_forms=1
    )
    blinded_bits = min(bits for _, bits in blinded_terms)
    blinding_bits = min(bits for _, bits in blinding_terms)
    return {
        "num_variables": num_variables,
        "num_blinding_variables": num_blinding_variables,
        "q_delta_1": q_delta_1,
        "q_delta_2": q_delta_2,
        "query_upper_bound": query_upper_bound,
        "blinded_bits": blinded_bits,
        "blinded_bottleneck": min(blinded_terms, key=lambda item: item[1]),
        "blinding_bits": blinding_bits,
        "blinding_bottleneck": min(blinding_terms, key=lambda item: item[1]),
        "max_requested_pow_bits": max(
            blinded.max_requested_pow_bits(), blinding.max_requested_pow_bits()
        ),
        "total_bits": min(blinded_bits, blinding_bits),
        "floor_bits": math.floor(min(blinded_bits, blinding_bits)),
    }


def estimate_provekit(json_path: Path) -> dict:
    measurements = load_measurements(json_path)
    provekit_constraints = [
        item["num_constraints"] for item in measurements if item["system"] == "provekit"
    ]
    if not provekit_constraints:
        raise ValueError(f"no provekit measurements found in {json_path}")

    m0_values = [ceil_log2(c) for c in provekit_constraints]
    checked_nv_min = 13
    checked_nv_max = max(24, max(m0_values) + 3)
    by_nv = [
        provekit_whir_security_for_nv(nv)
        for nv in range(checked_nv_min, checked_nv_max + 1)
    ]
    worst = min(by_nv, key=lambda item: item["total_bits"])

    return {
        "constraint_min": min(provekit_constraints),
        "constraint_max": max(provekit_constraints),
        "m0_min": min(m0_values),
        "m0_max": max(m0_values),
        "checked_nv_min": checked_nv_min,
        "checked_nv_max": checked_nv_max,
        "pow_bits": 10,
        "protocol_security_level": 118,
        "field_bits": field_size_bits(BN254_SCALAR_MODULUS),
        "initial_folding_factor": 3,
        "folding_factor": 3,
        "starting_log_inv_rate": 2,
        "worst": worst,
        "total_bits": worst["total_bits"],
        "floor_bits": worst["floor_bits"],
    }


def estimate_risc0() -> dict:
    # Mirrored from the configured RISC Zero toy-model security calculation for
    # the default benchmark segment size.
    queries = 50
    inv_rate = 4
    rho = 1.0 / inv_rate

    w_accum = 103.0
    w_code = 1.0
    w_data = 211.0
    n_trace_polys = w_accum + w_code + w_data
    max_degree = 5.0
    ext_size = 4
    cycles = float(1 << 20)
    trace_domain_size = cycles
    ext_field_size = float(BABY_BEAR) ** ext_size

    # Mirrored from local soundness.rs: permutation / lookup and plain
    # constraint terms in the configured toy-model path.
    plonk_plookup_error = (
        w_accum / ext_size * (max_degree - 2.0) * trace_domain_size / ext_field_size
    )
    constraints_error = 1.0 / ext_field_size
    toy_old_total_error = plonk_plookup_error + constraints_error + rho**queries
    toy_model_bits = abs(math.log2(toy_old_total_error))

    return {
        "queries": queries,
        "rho": rho,
        "field_q": BABY_BEAR,
        "cycles_po2": 20,
        "trace_domain_size": trace_domain_size,
        "ext_field_size": ext_field_size,
        "n_trace_polys": n_trace_polys,
        "w_accum": w_accum,
        "w_code": w_code,
        "w_data": w_data,
        "toy_model": {
            "bits": toy_model_bits,
            "floor_bits": math.floor(toy_model_bits),
        },
    }


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--bench-json",
        default="results/collected_benchmarks_26644954256.json",
        help="benchmark JSON artifact to read for provekit constraints",
    )
    args = parser.parse_args()
    bench_payload = json.loads(Path(args.bench_json).read_text())
    metadata_bits = {
        system: props["security_bits"]
        for system, props in bench_payload.get("systems", {}).items()
    }

    fri_systems = [
        {
            "system": "cairo-m",
            "q": M31,
            "log_blowup": 1,
            "queries": 80,
            "pow_bits": 16,
        },
        {
            "system": "miden",
            "q": GOLDILOCKS,
            "log_blowup": 3,
            "queries": 27,
            "pow_bits": 16,
        },
        {
            "system": "plonky2",
            "q": GOLDILOCKS,
            "log_blowup": 3,
            "queries": 28,
            "pow_bits": 16,
        },
        {
            "system": "rookie-numbers",
            "q": M31,
            "log_blowup": 1,
            "queries": 70,
            "pow_bits": 26,
        },
    ]

    print("Applicable benchmark estimates")
    print("| System | Metadata bits | Estimate | Floor | Calculation |")
    print("| --- | --- | ---: | ---: | --- |")
    for system in fri_systems:
        estimate = estimate_fri_bits(
            q=system["q"],
            log_blowup=system["log_blowup"],
            queries=system["queries"],
            pow_bits=system["pow_bits"],
        )
        calculation = (
            f'{system["pow_bits"]} + {system["queries"]} * '
            f'{estimate["bits_per_query"]:.12f}'
        )
        print(
            f'| {system["system"]} | {metadata_bits.get(system["system"], "n/a")} | '
            f'{estimate["total_bits"]:.6f} | {estimate["floor_bits"]} | '
            f"{calculation} |"
        )

    provekit = estimate_provekit(Path(args.bench_json))
    print(
        f"| provekit | {metadata_bits.get('provekit', 'n/a')} | {provekit['total_bits']:.6f} | "
        f"{provekit['floor_bits']} | pinned Johnson-bound WHIR config |"
    )

    stark_v = estimate_stark_v()
    print(
        f"| stark-v | {metadata_bits.get('stark-v', 'n/a')} | "
        f"{stark_v['total_bits']:.6f} | {stark_v['floor_bits']} | "
        "upstream secure_pcs_config UDR; batching bottleneck |"
    )

    risc0 = estimate_risc0()
    print(
        f"| risc0 | {metadata_bits.get('risc0', 'n/a')} | "
        f"{risc0['toy_model']['bits']:.6f} | "
        f"{risc0['toy_model']['floor_bits']} | "
        "RISC Zero toy-model reproduction; official target is 96 |"
    )

    print()
    print("ProveKit / WHIR")
    print(
        f"- constraints range: {provekit['constraint_min']}..{provekit['constraint_max']}"
    )
    print(f"- m0 range: {provekit['m0_min']}..{provekit['m0_max']}")
    print(
        f"- checked witness-variable range: {provekit['checked_nv_min']}.."
        f"{provekit['checked_nv_max']}"
    )
    print(f"- field_bits: {provekit['field_bits']:.12f}")
    print(f"- pow_bits: {provekit['pow_bits']}")
    print(f"- protocol_security_level: {provekit['protocol_security_level']}")
    print(
        "- WHIR params: "
        "unique_decoding=false, starting_log_inv_rate="
        f"{provekit['starting_log_inv_rate']}, initial_folding_factor="
        f"{provekit['initial_folding_factor']}, folding_factor="
        f"{provekit['folding_factor']}, batch_size=1"
    )
    print(
        "- worst checked witness config: "
        f"m={provekit['worst']['num_variables']}, "
        f"ell={provekit['worst']['num_blinding_variables']}, "
        f"q_delta_1={provekit['worst']['q_delta_1']}, "
        f"q_delta_2={provekit['worst']['q_delta_2']}, "
        f"query_upper_bound={provekit['worst']['query_upper_bound']}"
    )
    print(
        "- bottlenecks: "
        f"witness {provekit['worst']['blinded_bottleneck'][0]}="
        f"{provekit['worst']['blinded_bottleneck'][1]:.6f}, "
        f"blinding {provekit['worst']['blinding_bottleneck'][0]}="
        f"{provekit['worst']['blinding_bottleneck'][1]:.6f}"
    )
    print(
        f"- max requested PoW bits in checked range: "
        f"{provekit['worst']['max_requested_pow_bits']:.6f}"
    )
    print(
        f"- configured estimate: {provekit['total_bits']:.6f} "
        f"(floor {provekit['floor_bits']})"
    )

    print()
    print("Stark-V")
    print(
        f"- upstream UDR batching phase: {stark_v['batching_bits']:.6f} bits "
        f"(floor {math.floor(stark_v['batching_bits'])})"
    )
    print(
        f"- upstream UDR query phase: {stark_v['query_bits']:.6f} bits "
        f"(floor {math.floor(stark_v['query_bits'])})"
    )
    print(
        "- upstream UDR commit rounds: "
        + ", ".join(f"{bits:.6f}" for bits in stark_v["commit_bits"])
    )
    print(
        f"- configured estimate: {stark_v['total_bits']:.6f} "
        f"(floor {stark_v['floor_bits']})"
    )

    print()
    print("RISC Zero")
    print(
        f"- configured toy-model reproduction: {risc0['toy_model']['bits']:.6f} bits"
    )
    print(
        "- metadata target: 96 bits; the local reproduction is above that target "
        "for the default segment_po2=20 benchmark config"
    )


if __name__ == "__main__":
    main()
