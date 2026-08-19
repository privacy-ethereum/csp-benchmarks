pragma circom 2.0.2;

/*
    Signed multi-precision integers for witness-generation ("function land").

    Representation: var[K+1]
        index 0      : sign, 0 = non-negative, 1 = negative
        index 1..K   : magnitude, K limbs of N bits, little-endian

    Zero always carries sign 0, so representations are unique.

    bigint_func.circom is unsigned throughout and has no addition, so both the
    sign handling and long_add live here.
*/

include "./bigint_func.circom";

// Missing from bigint_func.circom. Returns K+1 limbs; the caller must size K
// with enough headroom that the top limb never overflows.
function sgn_long_add(N, K, a, b) {
    var out[100];
    var carry = 0;
    for (var i = 0; i < K; i++) {
        var t = a[i] + b[i] + carry;
        if (t >= (1 << N)) { out[i] = t - (1 << N); carry = 1; }
        else               { out[i] = t;            carry = 0; }
    }
    out[K] = carry;
    return out;
}

function sgn_zero(K) {
    var out[100];
    for (var i = 0; i <= K; i++) out[i] = 0;
    return out;
}

function sgn_is_zero(K, a) {
    for (var i = 1; i <= K; i++) { if (a[i] != 0) return 0; }
    return 1;
}

function sgn_neg(K, a) {
    var out[100];
    for (var i = 0; i <= K; i++) out[i] = a[i];
    if (sgn_is_zero(K, a) == 0) out[0] = 1 - a[0];
    return out;
}

// 0 if |a| < |b|, 1 if equal, 2 if |a| > |b|.
// long_gt returns 1 only for strict greater-than, so equality needs both calls.
function sgn_cmp_abs(N, K, a, b) {
    var am[100]; var bm[100];
    for (var i = 0; i < K; i++) { am[i] = a[i + 1]; bm[i] = b[i + 1]; }
    if (long_gt(N, K, am, bm) == 1) return 2;
    if (long_gt(N, K, bm, am) == 1) return 0;
    return 1;
}

function sgn_add(N, K, a, b) {
    var out[100];
    var am[100]; var bm[100];
    for (var i = 0; i < K; i++) { am[i] = a[i + 1]; bm[i] = b[i + 1]; }

    if (a[0] == b[0]) {
        var s[100] = sgn_long_add(N, K, am, bm);
        out[0] = a[0];
        for (var i = 0; i < K; i++) out[i + 1] = s[i];
        // s[K] must be 0; K is sized so this holds. Left unchecked on purpose:
        // an assert here would fire during witness generation with no context.
        if (sgn_is_zero(K, out) == 1) out[0] = 0;
        return out;
    }

    var c = sgn_cmp_abs(N, K, a, b);
    // circom 2.0.2 miscompiles a bare (brace-less) `if (...) return <call>;`
    // followed by more statements — it panics in translate.rs with
    // "This case should be unreachable". Braces avoid the parser/IR path
    // that trips it; every branch below stays braced for the same reason.
    if (c == 1) { return sgn_zero(K); }
    if (c == 2) {
        var d[100] = long_sub(N, K, am, bm);
        out[0] = a[0];
        for (var i = 0; i < K; i++) out[i + 1] = d[i];
    } else {
        var d2[100] = long_sub(N, K, bm, am);
        out[0] = b[0];
        for (var i = 0; i < K; i++) out[i + 1] = d2[i];
    }
    return out;
}

function sgn_sub(N, K, a, b) {
    return sgn_add(N, K, a, sgn_neg(K, b));
}

function sgn_mul(N, K, a, b) {
    var out[100];
    var am[100]; var bm[100];
    for (var i = 0; i < K; i++) { am[i] = a[i + 1]; bm[i] = b[i + 1]; }
    var p[100] = prod(N, K, am, bm);        // 2K limbs
    for (var i = 0; i < K; i++) out[i + 1] = p[i];
    // Limbs K..2K-1 of the product must be zero; K is sized for that.
    out[0] = 0;
    if (a[0] != b[0]) out[0] = 1;
    if (sgn_is_zero(K, out) == 1) out[0] = 0;
    return out;
}

// Index of the highest non-zero magnitude limb, 0 when the value is zero.
// long_div requires its divisor's top limb to be non-zero, so every division
// must be issued at the divisor's true length, not at K.
function sgn_top_limb(K, a) {
    for (var i = K; i >= 1; i--) { if (a[i] != 0) return i - 1; }
    return 0;
}

// Round half away from zero: a tie rounds to the larger magnitude, never
// toward zero. The lattice reduction that consumes this depends on that rule.
// Requires |b| != 0: long_div divides by the divisor's top limb internally
// (inside short_div/short_div_norm), so a zero divisor divides by zero there.
function sgn_divround(N, K, a, b) {
    var out[100];
    var am[100]; var bm[100];
    for (var i = 0; i < K; i++) { am[i] = a[i + 1]; bm[i] = b[i + 1]; }

    var kb = sgn_top_limb(K, b) + 1;        // true limb count of the divisor
    var m = K - kb;
    var dr[2][100] = long_div(N, kb, m, am, bm);

    // long_div only writes quotient limbs 0..m and remainder limbs 0..kb, so
    // everything above them has to be zeroed before it is read at width K.
    var q[100]; var r[100];
    for (var i = 0; i < K; i++) {
        q[i] = 0;
        r[i] = 0;
        if (i <= m)  q[i] = dr[0][i];
        if (i <= kb) r[i] = dr[1][i];
    }

    // 2*|r| >= |b|  ->  round away from zero
    var r2[100] = sgn_long_add(N, K, r, r);
    // r2 (2*|r|) carries K+1 limbs from sgn_long_add; compare at K+1 limbs
    // too, so a carry out of the top limb still participates in the
    // 2|r| >= |b| decision instead of silently wrapping.
    var bmx[100];
    for (var i = 0; i < K; i++) { bmx[i] = bm[i]; }
    bmx[K] = 0;
    var bumped = 0;
    if (long_gt(N, K + 1, bmx, r2) == 0) { bumped = 1; }   // !(|b| > 2|r|)  =>  2|r| >= |b|

    if (bumped == 1) {
        var one[100];
        for (var i = 0; i < K; i++) one[i] = 0;
        one[0] = 1;
        q = sgn_long_add(N, K, q, one);
    }

    for (var i = 0; i < K; i++) out[i + 1] = q[i];
    out[0] = 0;
    if (a[0] != b[0]) out[0] = 1;
    if (sgn_is_zero(K, out) == 1) out[0] = 0;
    return out;
}
