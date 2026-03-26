#!/usr/bin/env python3
"""Re-estimate default-CI STARK / FRI benchmark security after the 2025 papers.

This script keeps the calculations used in
results/stark_fri_security_report.md in one reproducible place.

The primary outputs are the paper-based modified-conjecture re-estimates.
For RISC Zero, the script also reproduces the upstream strict / toy-model
numbers and the local proven floor for comparison.

Reference map for the paper-derived substitutions:

- Crites-Stewart 2025, "On Reed-Solomon Proximity Gaps Conjectures"
  (IACR ePrint 2025/2046, https://eprint.iacr.org/2025/2046)
  - Theorem 7.4.1 and the displayed definition of H_q immediately below it:
    q-ary entropy / list-decoding capacity. In the pdftotext extraction used
    locally, this is the block at lines 275-282.
  - Our Conjecture 1:
    prime-field replacement for DEEP-FRI / list-decoding thresholds. In the
    pdftotext extraction, this appears at lines 360-366.
  - Our Conjecture 2:
    prime-field replacement for correlated-agreement thresholds. In the
    pdftotext extraction, this appears at lines 1053-1060.
  - Our Conjecture 3:
    prime-field replacement for WHIR mutual-correlated-agreement thresholds. In
    the pdftotext extraction, this appears at lines 1110-1112.
  - Introduction paragraph:
    the "loss of 1 / log2 q in the error rate" summary is the paragraph at
    lines 83-86.

- Diamond-Gruen 2025, "On the Distribution of the Distances of Random Words"
  (IACR ePrint 2025/2010, https://eprint.iacr.org/2025/2010)
  - Used only qualitatively in the report to justify rejecting the old
    unamended capacity-style conjectures. No numeric formula in this script is
    taken directly from that paper. The report's qualitative reference is to
    amended Conjecture 5.1, which appears at lines 3224-3233 in the local
    pdftotext extraction.

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


def ceil_log2(n: int) -> int:
    if n <= 1:
        return 0
    return (n - 1).bit_length()


def load_measurements(path: Path) -> list[dict]:
    payload = json.loads(path.read_text())
    return payload["measurements"]


def estimate_provekit(json_path: Path) -> dict:
    measurements = load_measurements(json_path)
    provekit_constraints = [
        item["num_constraints"] for item in measurements if item["system"] == "provekit"
    ]
    if not provekit_constraints:
        raise ValueError(f"no provekit measurements found in {json_path}")

    m0_values = [ceil_log2(c) for c in provekit_constraints]
    nv_values = [max(ceil_log2(4 * m0) + 1, 12) for m0 in m0_values]
    unique_nv = sorted(set(nv_values))
    if unique_nv != [12]:
        raise ValueError(f"unexpected provekit num_variables set: {unique_nv}")

    security_level = 128
    num_variables = 12
    starting_log_inv_rate = 1
    folding_factor = 4
    field_q = BN254_SCALAR_MODULUS
    field_bits = field_q.bit_length()

    # These two lines are mirrored from pinned WHIR / ProveKit code:
    # default_max_pow(num_variables, 1) and protocol_security_level =
    # security_level - pow_bits. They are not formulas from either supplied
    # paper.
    pow_bits = num_variables + starting_log_inv_rate - 3
    protocol_security_level = security_level - pow_bits

    def paper_eta(log_inv_rate: int, code_length: int) -> dict:
        rho = 2.0 ** (-log_inv_rate)
        # Mirrors WHIR's local ConjectureList choice:
        # log_eta = -(r + 1), equivalently eta_old = 2^{-(r + 1)}.
        # This relation comes from whir/src/whir/parameters.rs, not the papers.
        eta_old = 2.0 ** (-(log_inv_rate + 1))
        delta_old = 1.0 - rho - eta_old
        # Paper-driven substitution:
        # Crites-Stewart 2025, Our Conjecture 3 says the WHIR
        # mutual-correlated-agreement threshold becomes
        # H_q(delta) < 1 - 1/n - rho - eta for prime fields;
        # pdftotext lines 1110-1112.
        #
        # The script keeps WHIR's old boundary distance delta_old fixed and
        # solves for the admissible replacement eta_new:
        #   eta_new = 1 - 1/n - rho - H_q(delta_old)
        #
        # That algebraic rearrangement is our inference from Our Conjecture 3;
        # the paper states the threshold condition, not this WHIR-specific
        # "solve for eta while holding delta_old fixed" step.
        eta_new = 1.0 - 1.0 / code_length - rho - h_q(delta_old, field_q)
        if eta_new <= 0.0:
            raise ValueError(
                f"paper eta became non-positive for rate={log_inv_rate}, n={code_length}"
            )
        return {
            "rho": rho,
            "eta_old": eta_old,
            "delta_old": delta_old,
            "eta_new": eta_new,
            "log_eta_new": math.log2(eta_new),
            "code_length": code_length,
        }

    def list_size_bits(nv: int, log_inv_rate: int, log_eta: float) -> float:
        # Mirrored exactly from WHIR's list_size_bits(ConjectureList, ...).
        # No corresponding new formula is introduced by the papers here; the
        # only paper-driven change is the replacement log_eta.
        return (nv + log_inv_rate) - log_eta

    def rbr_ood_sample(
        nv: int, log_inv_rate: int, log_eta: float, ood_samples: int
    ) -> float:
        # Mirrored from WHIR's rbr_ood_sample. No direct paper formula; this is
        # the local bound being re-evaluated at the paper-updated eta.
        list_bits = list_size_bits(nv, log_inv_rate, log_eta)
        error = 2.0 * list_bits + nv * ood_samples
        return ood_samples * field_bits + 1.0 - error

    def ood_samples(nv: int, log_inv_rate: int, log_eta: float) -> int:
        for samples in range(1, 64):
            if rbr_ood_sample(nv, log_inv_rate, log_eta, samples) >= security_level:
                return samples
        raise ValueError(
            f"could not satisfy OOD sample requirement for nv={nv}, rate={log_inv_rate}"
        )

    def fold_prox_bits(nv: int, log_inv_rate: int, log_eta: float) -> float:
        # Mirrored from WHIR's rbr_soundness_fold_prox_gaps for ConjectureList.
        # Again, the paper only changes the admissible threshold via eta.
        error = (nv + log_inv_rate) - log_eta
        return field_bits - error

    def fold_sumcheck_bits(nv: int, log_inv_rate: int, log_eta: float) -> float:
        # Mirrored from WHIR's rbr_soundness_fold_sumcheck.
        return field_bits - (list_size_bits(nv, log_inv_rate, log_eta) + 1.0)

    def queries(log_inv_rate: int) -> int:
        # Mirrored from WHIR's queries(ConjectureList, ...). This is code
        # behavior, not a paper formula.
        return math.ceil(protocol_security_level / log_inv_rate)

    def rbr_queries(log_inv_rate: int, num_queries: int) -> float:
        # Mirrored from WHIR's rbr_queries(ConjectureList, ...).
        return num_queries * log_inv_rate

    def query_combination_bits(
        nv: int,
        log_inv_rate: int,
        log_eta: float,
        num_ood_samples: int,
        num_queries: int,
    ) -> float:
        # Mirrored from WHIR's rbr_soundness_queries_combination.
        list_bits = list_size_bits(nv, log_inv_rate, log_eta)
        log_combination = math.log2(num_ood_samples + num_queries)
        return field_bits - (log_combination + list_bits + 1.0)

    start_eta = paper_eta(
        starting_log_inv_rate, 2 ** (num_variables + starting_log_inv_rate)
    )
    start_ood_samples = ood_samples(
        num_variables, starting_log_inv_rate, start_eta["log_eta_new"]
    )
    start_commitment_bits = rbr_ood_sample(
        num_variables,
        starting_log_inv_rate,
        start_eta["log_eta_new"],
        start_ood_samples,
    )
    start_folding_bits = min(
        fold_prox_bits(num_variables, starting_log_inv_rate, start_eta["log_eta_new"]),
        fold_sumcheck_bits(
            num_variables, starting_log_inv_rate, start_eta["log_eta_new"]
        ),
    )

    round_details = []
    current_num_variables = num_variables - folding_factor
    current_log_inv_rate = starting_log_inv_rate
    num_rounds = num_variables // folding_factor - 1
    for _ in range(num_rounds):
        next_rate = current_log_inv_rate + (folding_factor - 1)
        code_length = 2 ** (current_num_variables + next_rate)
        eta = paper_eta(next_rate, code_length)
        num_queries = queries(current_log_inv_rate)
        num_ood = ood_samples(current_num_variables, next_rate, eta["log_eta_new"])
        query_bits = rbr_queries(current_log_inv_rate, num_queries)
        combination_bits = query_combination_bits(
            current_num_variables,
            next_rate,
            eta["log_eta_new"],
            num_ood,
            num_queries,
        )
        pow_added = max(security_level - min(query_bits, combination_bits), 0.0)
        total_bits = min(query_bits, combination_bits) + pow_added
        folding_bits = min(
            fold_prox_bits(current_num_variables, next_rate, eta["log_eta_new"]),
            fold_sumcheck_bits(current_num_variables, next_rate, eta["log_eta_new"]),
        )
        round_details.append(
            {
                "old_rate": current_log_inv_rate,
                "next_rate": next_rate,
                "num_variables": current_num_variables,
                "code_length": code_length,
                "delta_old": eta["delta_old"],
                "eta_old": eta["eta_old"],
                "eta_new": eta["eta_new"],
                "log_eta_new": eta["log_eta_new"],
                "ood_samples": num_ood,
                "queries": num_queries,
                "query_bits": query_bits,
                "combination_bits": combination_bits,
                "pow_added": pow_added,
                "total_bits": total_bits,
                "folding_bits": folding_bits,
            }
        )
        current_num_variables -= folding_factor
        current_log_inv_rate = next_rate

    final_queries = queries(current_log_inv_rate)
    final_query_bits = rbr_queries(current_log_inv_rate, final_queries)
    final_pow_bits = max(security_level - final_query_bits, 0.0)
    final_total_bits = final_query_bits + final_pow_bits

    candidates = [start_commitment_bits, start_folding_bits, final_total_bits]
    candidates.extend(round_info["total_bits"] for round_info in round_details)
    candidates.extend(round_info["folding_bits"] for round_info in round_details)
    total_bits = min(candidates)

    return {
        "constraint_min": min(provekit_constraints),
        "constraint_max": max(provekit_constraints),
        "m0_min": min(m0_values),
        "m0_max": max(m0_values),
        "num_variables": num_variables,
        "pow_bits": pow_bits,
        "protocol_security_level": protocol_security_level,
        "field_bits": field_bits,
        "start": {
            "rate": starting_log_inv_rate,
            "code_length": start_eta["code_length"],
            "delta_old": start_eta["delta_old"],
            "eta_old": start_eta["eta_old"],
            "eta_new": start_eta["eta_new"],
            "ood_samples": start_ood_samples,
            "commitment_bits": start_commitment_bits,
            "folding_bits": start_folding_bits,
        },
        "rounds": round_details,
        "final": {
            "rate": current_log_inv_rate,
            "queries": final_queries,
            "query_bits": final_query_bits,
            "pow_bits": final_pow_bits,
            "total_bits": final_total_bits,
        },
        "total_bits": total_bits,
        "floor_bits": math.floor(total_bits),
    }


def estimate_risc0() -> dict:
    # The local constants in this function are pinned-code parameters from
    # risc0-zkp / risc0-zkvm / risc0-circuit-rv32im. The papers only justify
    # the threshold substitutions below; they do not supply these circuit
    # constants.
    queries = 50
    inv_rate = 4
    rho = 1.0 / inv_rate
    eta = 0.05
    fri_fold = 16
    fri_min_degree = 256
    m = 16.0

    w_accum = 103.0
    w_code = 1.0
    w_data = 211.0
    n_trace_polys = w_accum + w_code + w_data
    max_degree = 5.0
    num_segment_polynomials = max_degree - 1.0
    biggest_combo = 6.0
    ext_size = 4
    cycles = float(1 << 20)
    trace_domain_size = cycles
    lde_domain_size = trace_domain_size * inv_rate
    ext_field_size = float(BABY_BEAR) ** ext_size

    coeffs_size = int(cycles) * ext_size
    num_folding_rounds = 0
    coeffs_cursor = coeffs_size
    while coeffs_cursor / ext_size > fri_min_degree:
        coeffs_cursor /= fri_fold
        num_folding_rounds += 1

    # Mirrored from local soundness.rs: permutation / lookup and plain
    # constraint terms. These are not changed by either paper.
    plonk_plookup_error = (
        w_accum / ext_size * (max_degree - 2.0) * trace_domain_size / ext_field_size
    )
    constraints_error = 1.0 / ext_field_size

    def e_fri_queries(theta: float) -> float:
        # Mirrored from local e_fri_queries(theta) = (1 - theta)^queries.
        return (1.0 - theta) ** queries

    def e_proximity_gap_proven() -> float:
        # Mirrored from local proven() / e_proximity_gap_proven().
        # This is not a formula from the two supplied papers; it is retained
        # only to reproduce the local conservative floor for context.
        return (
            (m + 0.5) ** 7
            / (3.0 * math.sqrt(rho) ** 3)
            * (lde_domain_size**2 / ext_field_size)
        )

    def e_proximity_gap_conjectured() -> float:
        # Mirrored from local conjectured_strict() / e_proximity_gap_conjectured().
        # Crites-Stewart 2025, Our Conjecture 2 changes the admissible
        # theta-threshold (pdftotext lines 1053-1060), but the report
        # intentionally keeps this error-term shape unchanged.
        first_term = 1.0 / (eta * rho)
        second_term = (n_trace_polys * lde_domain_size) / ext_field_size
        return first_term * second_term

    def e_fri_constant(e_proximity_gap: float) -> float:
        # Mirrored from local e_fri_constant(). No direct paper formula.
        first_term = (n_trace_polys + num_segment_polynomials - 0.5) * e_proximity_gap
        second_term = (
            (2.0 * m + 1.0)
            * (lde_domain_size + 1.0)
            * (fri_fold * num_folding_rounds)
            / (math.sqrt(rho) * ext_field_size)
        )
        return first_term + second_term

    def e_ali(l_plus: float) -> float:
        # Mirrored from local e_ali(). No direct paper formula.
        return l_plus * n_trace_polys / ext_field_size

    def e_deep(l_plus: float) -> float:
        # Mirrored from local e_deep(). No direct paper formula.
        h_plus = trace_domain_size + biggest_combo
        numerator = num_segment_polynomials * (h_plus - 1.0) + (trace_domain_size - 1.0)
        denominator = ext_field_size - trace_domain_size - lde_domain_size
        return l_plus * numerator / denominator

    def soundness_bits(theta: float, e_proximity_gap: float, l_plus: float) -> float:
        # Standard "bits = -log2(total_error)" conversion, using the local
        # decomposition of error terms from soundness.rs.
        total_error = (
            plonk_plookup_error
            + (e_fri_constant(e_proximity_gap) + e_fri_queries(theta))
            + e_deep(l_plus)
            + e_ali(l_plus)
        )
        return abs(math.log2(total_error))

    # Paper-driven toy-model update:
    # - Crites-Stewart 2025, Theorem 7.4.1 provides the H_q capacity line;
    #   pdftotext lines 275-282.
    # - Crites-Stewart 2025, Our Conjecture 1 provides the prime-field H_q
    #   replacement; pdftotext lines 360-366.
    # - We infer delta* from H_q(delta*) = 1 - rho and then replace the local
    #   toy-model FRI term rho^queries by rho_eff^queries with rho_eff = 1 - delta*.
    # This "replace rho by rho_eff inside toy_model_security()" step is an
    # inference used by the report, not a verbatim paper formula.
    toy_delta_star = invert_h_q(1.0 - rho, BABY_BEAR)
    toy_rho_eff = 1.0 - toy_delta_star
    toy_old_total_error = plonk_plookup_error + constraints_error + rho**queries
    toy_new_total_error = plonk_plookup_error + constraints_error + toy_rho_eff**queries

    # Local pre-paper threshold from soundness.rs:
    # theta_old = 1 - rho - eta.
    theta_old = 1.0 - rho - eta
    # Paper-driven strict-path update:
    # Crites-Stewart 2025, Our Conjecture 2 says the correlated-agreement
    # threshold becomes delta <= 1 - H_q(delta) - 1/n - eta for prime fields;
    # pdftotext lines 1053-1060.
    #
    # The script keeps the old right-hand side fixed and solves
    # H_q(theta_new) = 1 - 1/n - rho - eta.
    # This "solve for theta_new" step is our inference from Our Conjecture 2.
    theta_new = invert_h_q(1.0 - 1.0 / lde_domain_size - rho - eta, BABY_BEAR)
    rho_plus = (trace_domain_size + biggest_combo) / lde_domain_size
    epsilon_plus_old = 1.0 - rho_plus - theta_old
    # Paper-driven DEEP-FRI / list-decoding replacement:
    # Crites-Stewart 2025, Our Conjecture 1 replaces the old prime-field
    # list-decoding threshold by an H_q threshold; pdftotext lines 360-366.
    # The local epsilon_plus term is therefore updated by replacing theta_old
    # with H_q(theta_new). This is again an inference from the paper plus the
    # structure of local soundness.rs.
    epsilon_plus_new = 1.0 - rho_plus - h_q(theta_new, BABY_BEAR)
    l_plus_old = lde_domain_size / epsilon_plus_old
    l_plus_new = lde_domain_size / epsilon_plus_new

    # Mirrored from local proven() path. Kept only to reproduce the conservative
    # floor for context; not used as the primary paper-based estimate.
    alpha = (1.0 + 1.0 / (2.0 * m)) * math.sqrt(rho)
    theta_proven = 1.0 - alpha
    m_plus = 1.0 / (biggest_combo * (alpha / math.sqrt(rho_plus) - 1.0))
    l_plus_proven = (math.ceil(m_plus) + 0.5) / math.sqrt(rho_plus)

    strict_old_bits = soundness_bits(
        theta_old, e_proximity_gap_conjectured(), l_plus_old
    )
    strict_new_bits = soundness_bits(
        theta_new, e_proximity_gap_conjectured(), l_plus_new
    )
    proven_bits = soundness_bits(theta_proven, e_proximity_gap_proven(), l_plus_proven)

    return {
        "queries": queries,
        "rho": rho,
        "eta": eta,
        "field_q": BABY_BEAR,
        "cycles_po2": 20,
        "trace_domain_size": trace_domain_size,
        "lde_domain_size": lde_domain_size,
        "ext_field_size": ext_field_size,
        "n_trace_polys": n_trace_polys,
        "w_accum": w_accum,
        "w_code": w_code,
        "w_data": w_data,
        "biggest_combo": biggest_combo,
        "num_folding_rounds": num_folding_rounds,
        "toy_model": {
            "old_bits": abs(math.log2(toy_old_total_error)),
            "delta_star": toy_delta_star,
            "rho_eff": toy_rho_eff,
            "new_bits": abs(math.log2(toy_new_total_error)),
            "floor_bits": math.floor(abs(math.log2(toy_new_total_error))),
        },
        "strict": {
            "old_theta": theta_old,
            "new_theta": theta_new,
            "h_q_new_theta": h_q(theta_new, BABY_BEAR),
            "rho_plus": rho_plus,
            "epsilon_plus_old": epsilon_plus_old,
            "epsilon_plus_new": epsilon_plus_new,
            "old_bits": strict_old_bits,
            "new_bits": strict_new_bits,
            "floor_bits": math.floor(strict_new_bits),
        },
        "proven_floor": {
            "alpha": alpha,
            "theta": theta_proven,
            "bits": proven_bits,
            "floor_bits": math.floor(proven_bits),
        },
    }


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--bench-json",
        default="results/collected_benchmarks_23330555957.json",
        help="benchmark JSON artifact to read for provekit constraints",
    )
    args = parser.parse_args()

    fri_systems = [
        {
            "system": "cairo-m",
            "claim": "96",
            "q": M31,
            "log_blowup": 1,
            "queries": 80,
            "pow_bits": 16,
        },
        {
            "system": "miden",
            "claim": "128 benchmark / 96 locked",
            "q": GOLDILOCKS,
            "log_blowup": 3,
            "queries": 27,
            "pow_bits": 16,
        },
        {
            "system": "plonky2",
            "claim": "100",
            "q": GOLDILOCKS,
            "log_blowup": 3,
            "queries": 28,
            "pow_bits": 16,
        },
        {
            "system": "rookie-numbers",
            "claim": "96",
            "q": M31,
            "log_blowup": 1,
            "queries": 70,
            "pow_bits": 26,
        },
        {
            "system": "stark-v",
            "claim": "96",
            "q": M31,
            "log_blowup": 1,
            "queries": 70,
            "pow_bits": 26,
        },
    ]

    print("Paper-based primary estimates")
    print("| System | Claim | Estimate | Floor | Calculation |")
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
            f'| {system["system"]} | {system["claim"]} | '
            f'{estimate["total_bits"]:.6f} | {estimate["floor_bits"]} | '
            f"{calculation} |"
        )

    provekit = estimate_provekit(Path(args.bench_json))
    print(
        f"| provekit | 128 | {provekit['total_bits']:.6f} | "
        f"{provekit['floor_bits']} | pinned WHIR schedule + paper H_q replacement |"
    )

    risc0 = estimate_risc0()
    print(
        f"| risc0 | 96 base / 99 recursion | "
        f"{risc0['toy_model']['new_bits']:.6f} | "
        f"{risc0['toy_model']['floor_bits']} | toy-model FRI term with paper H_q replacement |"
    )

    print()
    print("ProveKit / WHIR")
    print(
        f"- constraints range: {provekit['constraint_min']}..{provekit['constraint_max']}"
    )
    print(f"- m0 range: {provekit['m0_min']}..{provekit['m0_max']}")
    print(f"- num_variables: {provekit['num_variables']}")
    print(f"- pow_bits: {provekit['pow_bits']}")
    print(f"- protocol_security_level: {provekit['protocol_security_level']}")
    print(
        "- start: "
        f"n={provekit['start']['code_length']}, "
        f"delta_old={provekit['start']['delta_old']:.12f}, "
        f"eta_old={provekit['start']['eta_old']:.12f}, "
        f"eta_paper={provekit['start']['eta_new']:.12f}, "
        f"OOD={provekit['start']['commitment_bits']:.6f}, "
        f"fold={provekit['start']['folding_bits']:.6f}"
    )
    print("- rounds:")
    for round_info in provekit["rounds"]:
        print(
            "  "
            f"old_rate={round_info['old_rate']}, next_rate={round_info['next_rate']}, "
            f"n={round_info['code_length']}, q={round_info['queries']}, "
            f"delta_old={round_info['delta_old']:.12f}, "
            f"eta_paper={round_info['eta_new']:.12f}, "
            f"query={round_info['query_bits']:.6f}, "
            f"combination={round_info['combination_bits']:.6f}, "
            f"pow={round_info['pow_added']:.6f}, "
            f"total={round_info['total_bits']:.6f}, "
            f"fold={round_info['folding_bits']:.6f}"
        )
    print(
        "- final: "
        f"rate={provekit['final']['rate']}, "
        f"q={provekit['final']['queries']}, "
        f"query={provekit['final']['query_bits']:.6f}, "
        f"pow={provekit['final']['pow_bits']:.6f}, "
        f"total={provekit['final']['total_bits']:.6f}"
    )
    print(
        f"- paper-based estimate: {provekit['total_bits']:.6f} "
        f"(floor {provekit['floor_bits']})"
    )

    print()
    print("RISC Zero")
    print(
        f"- toy-model upstream reproduction: {risc0['toy_model']['old_bits']:.6f} bits"
    )
    print(
        f"- toy-model paper-based update: {risc0['toy_model']['new_bits']:.6f} bits "
        f"(floor {risc0['toy_model']['floor_bits']})"
    )
    print(f"- strict upstream reproduction: {risc0['strict']['old_bits']:.6f} bits")
    print(
        f"- strict paper-based update: {risc0['strict']['new_bits']:.6f} bits "
        f"(floor {risc0['strict']['floor_bits']})"
    )
    print(
        f"- proven local floor: {risc0['proven_floor']['bits']:.6f} bits "
        f"(floor {risc0['proven_floor']['floor_bits']})"
    )
    print(
        "- strict update internals: "
        f"theta_old={risc0['strict']['old_theta']:.12f}, "
        f"theta_new={risc0['strict']['new_theta']:.12f}, "
        f"H_q(theta_new)={risc0['strict']['h_q_new_theta']:.12f}, "
        f"epsilon_plus_old={risc0['strict']['epsilon_plus_old']:.12f}, "
        f"epsilon_plus_new={risc0['strict']['epsilon_plus_new']:.12f}"
    )


if __name__ == "__main__":
    main()
