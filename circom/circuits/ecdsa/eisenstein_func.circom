pragma circom 2.0.2;

/*
    Eisenstein integers Z[omega], omega^2 = -1 - omega, for witness generation.

    Why they matter here: secp256k1's GLV endomorphism scalar satisfies
    lambda^2 + lambda + 1 == 0 (mod n), so omega |-> lambda is a ring
    homomorphism Z[omega] -> Z/n. n splits: there is pi with N(pi) = n. The
    4-dimensional fake-GLV decomposition then becomes a 2-dimensional Euclidean
    problem over Z[omega], which is a Euclidean domain.

    Technique from rot256's zk.golf record for secp256k1 variable-base scalar
    multiplication (challenge 05); pi's components match a2 and a1 in rot256's
    zk.golf challenge-05 submission.

    Representation: var[2*(K+1)] — first K+1 entries are component a0 in the
    signed_func layout, next K+1 are a1, for a = a0 + a1*omega.
*/

include "./signed_func.circom";

function eis_comp(K, a, i) {
    var out[100];
    var base = i * (K + 1);
    for (var j = 0; j <= K; j++) { out[j] = a[base + j]; }
    return out;
}

function eis_pack(K, c0, c1) {
    var out[100];
    for (var j = 0; j <= K; j++) { out[j] = c0[j]; out[K + 1 + j] = c1[j]; }
    return out;
}

function eis_is_zero(K, a) {
    if (sgn_is_zero(K, eis_comp(K, a, 0)) == 0) { return 0; }
    if (sgn_is_zero(K, eis_comp(K, a, 1)) == 0) { return 0; }
    return 1;
}

// circom 2.2.2's type inference cannot resolve a call to eis_pack whose two
// arguments are themselves function calls (confirmed by compiling this file:
// T2004 "Unable to infer the type of this function" on eis_sub/eis_conj/
// eis_div until rewritten this way; eis_mul, which already bound its two
// halves to local vars before packing, compiled clean). Calls to eis_pack
// below bind each component to a local var first, then pass plain variables —
// this is specific to eis_pack call sites; a nested call elsewhere (e.g.
// eis_norm's return below) is not affected and needs no such rewrite.
function eis_add(N, K, a, b) {
    var s0[100] = sgn_add(N, K, eis_comp(K, a, 0), eis_comp(K, b, 0));
    var s1[100] = sgn_add(N, K, eis_comp(K, a, 1), eis_comp(K, b, 1));
    return eis_pack(K, s0, s1);
}

function eis_sub(N, K, a, b) {
    var s0[100] = sgn_sub(N, K, eis_comp(K, a, 0), eis_comp(K, b, 0));
    var s1[100] = sgn_sub(N, K, eis_comp(K, a, 1), eis_comp(K, b, 1));
    return eis_pack(K, s0, s1);
}

// (a0 + a1 w)(b0 + b1 w) = (a0 b0 - a1 b1) + (a0 b1 + a1 b0 - a1 b1) w
function eis_mul(N, K, a, b) {
    var a0[100] = eis_comp(K, a, 0);
    var a1[100] = eis_comp(K, a, 1);
    var b0[100] = eis_comp(K, b, 0);
    var b1[100] = eis_comp(K, b, 1);

    var a0b0[100] = sgn_mul(N, K, a0, b0);
    var a1b1[100] = sgn_mul(N, K, a1, b1);
    var a0b1[100] = sgn_mul(N, K, a0, b1);
    var a1b0[100] = sgn_mul(N, K, a1, b0);

    var c0[100] = sgn_sub(N, K, a0b0, a1b1);
    var c1[100] = sgn_sub(N, K, sgn_add(N, K, a0b1, a1b0), a1b1);
    return eis_pack(K, c0, c1);
}

// conj(a0 + a1 w) = (a0 - a1) + (-a1) w
function eis_conj(N, K, a) {
    var a0[100] = eis_comp(K, a, 0);
    var a1[100] = eis_comp(K, a, 1);
    var c0[100] = sgn_sub(N, K, a0, a1);
    var c1[100] = sgn_neg(K, a1);
    return eis_pack(K, c0, c1);
}

// N(a) = a0^2 - a0 a1 + a1^2, always non-negative
function eis_norm(N, K, a) {
    var a0[100] = eis_comp(K, a, 0);
    var a1[100] = eis_comp(K, a, 1);
    var t[100] = sgn_sub(N, K, sgn_mul(N, K, a0, a0), sgn_mul(N, K, a0, a1));
    return sgn_add(N, K, t, sgn_mul(N, K, a1, a1));
}

// Euclidean quotient: round(a * conj(b) / N(b)) componentwise.
// Requires b != 0 — N(b) is then non-zero and sgn_divround's b != 0
// precondition is met. A caller that may see b == 0 (eis_is_zero(K, b) == 1)
// must branch around this function itself; eis_div does not guard it
// internally so that the zero check stays a single, explicit, visible
// decision at the call site rather than a silent fallback buried in here.
function eis_div(N, K, a, b) {
    var nb[100] = eis_norm(N, K, b);
    var cb[100] = eis_conj(N, K, b);
    var t[100] = eis_mul(N, K, a, cb);
    var q0[100] = sgn_divround(N, K, eis_comp(K, t, 0), nb);
    var q1[100] = sgn_divround(N, K, eis_comp(K, t, 1), nb);
    return eis_pack(K, q0, q1);
}

/*
    pi with N(pi) = n, computed once as gcd(n, lambda - omega) in Z[omega]:

      pi = 367917413016453100223835821029139468248
         +  64502973549206556628585045361533709077 * omega

    N(pi) is exactly n. The first component is 129 bits and needs three
    64-bit limbs; the second is 126 bits and needs two.
*/
function get_eisenstein_pi(N, K) {
    // The limbs below are hardcoded for N = 64 and need K >= 3 (c0[3] is
    // nonzero). Any other N would silently produce a wrong constant.
    assert(N == 64);
    var c0[100]; var c1[100];
    for (var i = 0; i <= K; i++) { c0[i] = 0; c1[i] = 0; }  // index 0 is the sign limb
    c0[1] = 6323353552219852760;    // 367917413016453100223835821029139468248 mod 2^64
    c0[2] = 1498098850674701302;    // >> 64
    c0[3] = 1;                      // >> 128
    c1[1] = 16747920425669159701;   //  64502973549206556628585045361533709077 mod 2^64
    c1[2] = 3496713202691238861;    // >> 64
    return eis_pack(K, c0, c1);
}
