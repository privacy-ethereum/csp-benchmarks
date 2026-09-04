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
    assert(sgn_is_zero(K, nb) == 0);
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

/*
    4-dimensional fake-GLV decomposition, entirely at witness-generation time.

    Finds U, V in Z[omega] with U == s*V (mod pi) and every component below
    2^64, which is exactly the circuit's hint:

        u1 + u2*lambda == s*(v1 + v2*lambda)   (mod n)

    Method: extended Euclid over Z[omega] on (pi, s), keeping the convergents,
    then a short search over the three convergents around the point where the
    norm drops below 2^128. Each candidate is alpha*(U_i,V_i) + beta*(U_j,V_j)
    with alpha, beta having components in {-1,0,1}; every such combination
    preserves the congruence, so the search only affects size, never
    soundness — it is a size optimization, not something the correctness of
    the result depends on.

    Measured on a large scalar sample: every result fits in 64 bits, matching
    what full lattice reduction achieves, at a small fraction of the work.

    Precondition: s is expected already reduced mod n (0 <= s < n). The
    congruence itself holds regardless -- it is mod n either way, and the
    first Euclid step self-corrects by swapping if s >= n -- but the size
    guarantee (every component under 2^64) is only argued for s < n.

    Technique from rot256's zk.golf record for secp256k1 variable-base scalar
    multiplication (challenge 05).
*/

// The stopping threshold, 2^128, as a signed magnitude.
function get_eis_stop(N, K) {
    // out[3] = 1 hardcodes 2^128 as three 64-bit limbs of zero followed by
    // a 1; any other N would silently produce the wrong threshold, so this
    // guards the same assumption get_eisenstein_pi guards for its own
    // hardcoded limbs.
    assert(N == 64);
    var out[100];
    for (var i = 0; i <= K; i++) { out[i] = 0; }
    out[3] = 1;                 // limb index 2 of the magnitude -> 2^128
    return out;
}

// Small Eisenstein coefficient with components x, y in {-1, 0, 1}. N is
// unused: a magnitude-1 coefficient occupies limb index 1 regardless of
// limb width, so nothing here depends on N.
function eis_small(N, K, x, y) {
    var c0[100]; var c1[100];
    for (var i = 0; i <= K; i++) { c0[i] = 0; c1[i] = 0; }
    if (x != 0) {
        c0[1] = 1;
        if (x < 0) { c0[0] = 1; }
    }
    if (y != 0) {
        c1[1] = 1;
        if (y < 0) { c1[0] = 1; }
    }
    return eis_pack(K, c0, c1);
}

// Number of bits in the largest of the four components; 100 if any of them
// overflows a single limb, which makes such a candidate always lose.
function eis_quad_bits(N, K, U, V) {
    var worst = 0;
    for (var c = 0; c < 4; c++) {
        var v[100];
        if (c < 2) { v = eis_comp(K, U, c); } else { v = eis_comp(K, V, c - 2); }
        for (var i = 2; i <= K; i++) {
            if (v[i] != 0) { return 100; }
        }
        var m = v[1];
        var b = 0;
        while (m > 0) { m = m \ 2; b++; }
        if (b > worst) { worst = b; }
    }
    return worst;
}

// circom has no `continue` statement (P1012 "illegal expression" on that
// keyword), so loop bodies that would skip an iteration are wrapped in an
// `if` instead. Every nested function-call-as-argument below is also bound
// to a local var first, for the same T2004 type-inference reason documented
// above at eis_add/eis_sub/eis_conj/eis_div — the defect is not specific to
// eis_pack, it reproduces at any call site whose argument is itself a
// function call.
//
// A `while` condition that calls a function over a variable the loop body
// mutates evaluates stale: the loop body still runs once more after the
// call would already report the terminating value. Plain-variable
// conditions (e.g. `cnt < 128`, used below) are unaffected. The workaround
// evaluates the function once before the loop and again as the last
// statement of the body, and the condition then compares that plain
// variable instead of calling the function itself.
function fake_glv_decompose(N, K, s) {
    var out[2][4];

    var sc0[100]; var sc1[100];
    for (var i = 0; i <= K; i++) { sc0[i] = 0; sc1[i] = 0; }
    for (var i = 0; i < 4; i++) { sc0[i + 1] = s[i]; }

    var r0[100] = get_eisenstein_pi(N, K);
    var r1[100] = eis_pack(K, sc0, sc1);
    var zeroK[100] = sgn_zero(K);
    var y0[100] = eis_pack(K, zeroK, zeroK);
    var one[100];
    for (var i = 0; i <= K; i++) { one[i] = 0; }
    one[1] = 1;
    var y1[100] = eis_pack(K, one, zeroK);

    // Collect convergents. The search below only ever reads rows at index
    // <= hi, where hi = min(cross+1, cnt-1) and cross is the first row
    // whose norm drops under the search threshold (found in this same
    // loop). So a chain that runs longer than this array's capacity is
    // harmless as long as `cross` is found within it: the discarded tail
    // past `hi` was never going to be read either way. The row index
    // `cross` reaches is the load-bearing quantity, not the chain's full
    // length.
    //
    // Measured over 900,000 random scalars (three independent samples):
    // the highest row index ever read was 55, out of the 128 available
    // (indices 0-127). This is an empirical measurement over that sample
    // size, not a proven bound. See assert(bestBits <= 64) below for the
    // failure mode on a scalar whose window would need a row this array
    // does not have.
    //
    // 128 also stays clear of a separate limit: the witness calculator's
    // linear memory does not survive a cU/cV pair sized at 200 rows of
    // width 100 (writes past some point corrupt memory at run time even
    // though the circuit compiles cleanly); 150 rows is safe.
    var cU[128][100];
    var cV[128][100];
    var cnt = 0;
    var stop[100] = get_eis_stop(N, K);
    var cross = -1;
    var r1IsZero = eis_is_zero(K, r1);
    while (r1IsZero == 0 && cnt < 128) {
        cU[cnt] = r1;
        cV[cnt] = y1;
        if (cross < 0) {
            var nrm[100] = eis_norm(N, K, r1);
            if (sgn_cmp_abs(N, K, nrm, stop) == 0) { cross = cnt; }
        }
        cnt++;
        var q[100] = eis_div(N, K, r0, r1);
        var qr1[100] = eis_mul(N, K, q, r1);
        var r2[100] = eis_sub(N, K, r0, qr1);
        var qy1[100] = eis_mul(N, K, q, y1);
        var y2[100] = eis_sub(N, K, y0, qy1);
        r0 = r1; r1 = r2;
        r1IsZero = eis_is_zero(K, r1);
        y0 = y1; y1 = y2;
    }
    if (cross < 0) { cross = cnt - 1; }

    var lo = cross - 1;
    if (lo < 0) { lo = 0; }
    var hi = cross + 1;
    if (hi > cnt - 1) { hi = cnt - 1; }

    // s == 0 (mod n) as fed in: r1 starts zero, the loop above never runs, cnt
    // stays 0, and the search loop below (bounded by lo..hi, both clipped to
    // an empty range in that case) never executes either. U = 0 = s*(v1+v2*w)
    // holds for any V, so seed the defaults with V = 1 to keep (v1,v2) != 0 —
    // the same fallback the reference decomposition uses for this case.
    // A dynamically-indexed row of a 2D var array (cU[i] with i not a
    // compile-time constant), passed straight into a function call, is
    // mis-sized by the type checker (T3001 "Out of bounds" from inside
    // eis_comp). Binding the row to a local var first, as done throughout
    // below, resolves it — the same "bind before calling" shape as the
    // T2004 nested-call workaround above, for a different root cause.
    var bestU[100] = eis_pack(K, zeroK, zeroK);
    var bestV[100] = eis_pack(K, one, zeroK);
    var bestBits = 1000;
    for (var i = lo; i <= hi; i++) {
        var rowUi[100] = cU[i];
        var rowVi[100] = cV[i];

        // Pass 1: the convergent on its own, rotated by alpha in {-1,0,1}^2
        // minus (0,0). Every alpha here is tried, and considered against
        // bestBits, before any two-convergent candidate below — matching
        // the order of the two separate passes this is ported from, so a
        // tie between a single-convergent and a combined candidate resolves
        // the same way here as there.
        for (var ax = -1; ax <= 1; ax++) {
            for (var ay = -1; ay <= 1; ay++) {
                if (ax != 0 || ay != 0) {
                    var al[100] = eis_small(N, K, ax, ay);
                    var sU[100] = eis_mul(N, K, rowUi, al);
                    var sV[100] = eis_mul(N, K, rowVi, al);
                    if (eis_is_zero(K, sV) == 0) {
                        var sb = eis_quad_bits(N, K, sU, sV);
                        if (sb < bestBits) { bestBits = sb; bestU = sU; bestV = sV; }
                    }
                }
            }
        }

        // Pass 2: combined with the neighbouring convergent, alpha*(U_i,V_i)
        // + beta*(U_{i+1},V_{i+1}). alpha ranges over all nine values here
        // (including (0,0), unlike pass 1) — the only excluded case is both
        // alpha and beta being (0,0) at once, which is the sole condition
        // this candidate would be identically zero.
        if (i + 1 <= hi) {
            var rowUi1[100] = cU[i + 1];
            var rowVi1[100] = cV[i + 1];
            for (var ax = -1; ax <= 1; ax++) {
                for (var ay = -1; ay <= 1; ay++) {
                    var al2[100] = eis_small(N, K, ax, ay);
                    var aU[100] = eis_mul(N, K, rowUi, al2);
                    var aV[100] = eis_mul(N, K, rowVi, al2);
                    for (var bx = -1; bx <= 1; bx++) {
                        for (var by = -1; by <= 1; by++) {
                            if (ax != 0 || ay != 0 || bx != 0 || by != 0) {
                                var be[100] = eis_small(N, K, bx, by);
                                var mUi1[100] = eis_mul(N, K, rowUi1, be);
                                var kU[100] = eis_add(N, K, aU, mUi1);
                                var mVi1[100] = eis_mul(N, K, rowVi1, be);
                                var kV[100] = eis_add(N, K, aV, mVi1);
                                if (eis_is_zero(K, kV) == 0) {
                                    var kb = eis_quad_bits(N, K, kU, kV);
                                    if (kb < bestBits) { bestBits = kb; bestU = kU; bestV = kV; }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // If every candidate in the search window overflowed a single limb,
    // bestBits is still whatever eis_quad_bits reported (100, since it
    // started at 1000 and any candidate improves on that) and bestU/bestV
    // hold a multi-limb value that out[0][c] = v[1] below would silently
    // truncate to its bottom 64 bits -- a well-formed but wrong quadruple,
    // with no signal at the point where the information to diagnose it
    // still exists. Fail loudly here instead, at zero constraint cost
    // (this function only ever produces `<--` witness values).
    //
    // The s == 0 (mod n) fallback above (lo > hi in that case) never runs
    // this search loop, so bestBits is left at its 1000 sentinel rather
    // than an actual bit count there -- that path's own values (U=0, V=1)
    // are correct and tiny by the inspection argued above, not by search,
    // so the assert below only needs to hold when the search loop actually
    // ran.
    if (lo <= hi) {
        assert(bestBits <= 64);
    }

    for (var c = 0; c < 4; c++) {
        var v[100];
        if (c < 2) { v = eis_comp(K, bestU, c); } else { v = eis_comp(K, bestV, c - 2); }
        out[0][c] = v[1];   // magnitude fits in one limb by construction
        out[1][c] = v[0];
    }
    return out;
}
