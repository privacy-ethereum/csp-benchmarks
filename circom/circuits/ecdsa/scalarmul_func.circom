pragma circom 2.0.2;

/*
    secp256k1 scalar multiplication at witness-generation time.

    Only used to derive R = [u1]G + [u2]Q so that Ry stops being a circuit
    input. Costs no constraints: every result reaches the circuit through a
    <-- assignment and is checked by the constraints that already exist.

    The accumulator is kept in Jacobian coordinates, x = X/Z^2 and y = Y/Z^3,
    so that no step needs a modular inversion: one inversion converts the
    result back to affine at the end of the whole multiplication. In affine
    coordinates every doubling and every addition inverts, which is the
    expensive operation here by a wide margin.

    The point at infinity is Z == 0. That representation also settles the two
    degenerate cases of addition rather than arguing them away: the mixed
    addition detects a repeated point and doubles instead, and detects a point
    added to its own negation and returns infinity.
*/

include "./secp256k1_func.circom";
include "./bigint_func.circom";

// secp256k1 generator point, split into four 64-bit limbs. get_gx/get_gy in
// secp256k1_func.circom only cover the (n=86, k=3) limb layout used by the
// base verifier; this module works in the (n=64, k=4) layout instead, so the
// generator is restated here in that layout rather than reused.
function get_gx64() {
    var ret[100];
    for (var i = 0; i < 100; i++) ret[i] = 0;
    ret[0] = 6481385041966929816;
    ret[1] = 188021827762530521;
    ret[2] = 6170039885052185351;
    ret[3] = 8772561819708210092;
    return ret;
}

function get_gy64() {
    var ret[100];
    for (var i = 0; i < 100; i++) ret[i] = 0;
    ret[0] = 11261198710074299576;
    ret[1] = 18237243440184513561;
    ret[2] = 6747795201694173352;
    ret[3] = 5204712524664259685;
    return ret;
}

// a mod p, for a of K registers.
//
// The helpers below reduce with a single conditional subtraction, which is
// correct only for operands already below p. The affine formulas this file
// replaced reduced by division on every operation and so accepted anything, so
// the input point is brought into range here rather than assumed to be.
function sm_reduce(N, K, a, p) {
    var out[100];
    for (var i = 0; i < 100; i++) {
        out[i] = 0;
    }

    var pGtA = long_gt(N, K, p, a);
    if (pGtA == 1) {
        for (var i = 0; i < K; i++) {
            out[i] = a[i];
        }
        return out;
    }

    var wide[100];
    for (var i = 0; i < 100; i++) {
        wide[i] = 0;
    }
    for (var i = 0; i < K; i++) {
        wide[i] = a[i];
    }
    var qr[2][100] = long_div(N, K, K, wide, p);
    for (var i = 0; i < K; i++) {
        out[i] = qr[1][i];
    }
    return out;
}

// 1 when every register of a below k is zero
function sm_is_zero(K, a) {
    var isZero = 1;
    for (var i = 0; i < K; i++) {
        if (a[i] != 0) {
            isZero = 0;
        }
    }
    return isZero;
}

// (a + b) mod p, for a, b < p. Addition is not available in bigint_func, and
// going through long_div for a sum that is at most 2p would cost more than the
// conditional subtraction below.
function sm_add_mod(N, K, a, b, p) {
    var pow = 1 << N;
    var s[100];
    for (var i = 0; i < 100; i++) {
        s[i] = 0;
    }

    var carry = 0;
    for (var i = 0; i < K; i++) {
        var t = a[i] + b[i] + carry;
        if (t >= pow) {
            s[i] = t - pow;
            carry = 1;
        } else {
            s[i] = t;
            carry = 0;
        }
    }

    // the sum is below 2p, so at most one subtraction reduces it. A carry out
    // of the top register means the sum is above 2^(N*K), hence above p.
    var needSub = carry;
    if (needSub == 0) {
        var pGtS = long_gt(N, K, p, s);
        if (pGtS == 0) {
            needSub = 1;
        }
    }
    if (needSub == 1) {
        var borrow = 0;
        for (var i = 0; i < K; i++) {
            if (s[i] >= p[i] + borrow) {
                s[i] = s[i] - p[i] - borrow;
                borrow = 0;
            } else {
                s[i] = pow + s[i] - p[i] - borrow;
                borrow = 1;
            }
        }
    }
    return s;
}

// (a - b) mod p, for a, b < p. long_sub_mod_p would serve, but it divides and
// it answers p rather than 0 when a == b, and the Jacobian formulas below test
// their differences against zero.
function sm_sub_mod(N, K, a, b, p) {
    var pow = 1 << N;
    var out[100];
    for (var i = 0; i < 100; i++) {
        out[i] = 0;
    }

    var bGtA = long_gt(N, K, b, a);
    if (bGtA == 0) {
        var borrow = 0;
        for (var i = 0; i < K; i++) {
            if (a[i] >= b[i] + borrow) {
                out[i] = a[i] - b[i] - borrow;
                borrow = 0;
            } else {
                out[i] = pow + a[i] - b[i] - borrow;
                borrow = 1;
            }
        }
    } else {
        var d[100];
        for (var i = 0; i < 100; i++) {
            d[i] = 0;
        }
        var borrowD = 0;
        for (var i = 0; i < K; i++) {
            if (b[i] >= a[i] + borrowD) {
                d[i] = b[i] - a[i] - borrowD;
                borrowD = 0;
            } else {
                d[i] = pow + b[i] - a[i] - borrowD;
                borrowD = 1;
            }
        }
        var borrowP = 0;
        for (var i = 0; i < K; i++) {
            if (p[i] >= d[i] + borrowP) {
                out[i] = p[i] - d[i] - borrowP;
                borrowP = 0;
            } else {
                out[i] = pow + p[i] - d[i] - borrowP;
                borrowP = 1;
            }
        }
    }
    return out;
}

// Doubling in Jacobian coordinates, for a curve with a == 0 such as this one.
// Infinity needs no special case: Z3 carries Z1 as a factor, so a zero Z stays
// zero.
// out[0] = X3, out[1] = Y3, out[2] = Z3
function sm_jac_double(N, K, X1, Y1, Z1, p) {
    var A[100] = prod_mod_p(N, K, X1, X1, p);
    var B[100] = prod_mod_p(N, K, Y1, Y1, p);
    var C[100] = prod_mod_p(N, K, B, B, p);

    var xb[100] = sm_add_mod(N, K, X1, B, p);
    var xbSq[100] = prod_mod_p(N, K, xb, xb, p);
    var dPre[100] = sm_sub_mod(N, K, xbSq, A, p);
    var dHalf[100] = sm_sub_mod(N, K, dPre, C, p);
    var D[100] = sm_add_mod(N, K, dHalf, dHalf, p);

    var A2[100] = sm_add_mod(N, K, A, A, p);
    var E[100] = sm_add_mod(N, K, A2, A, p);
    var F[100] = prod_mod_p(N, K, E, E, p);

    var D2[100] = sm_add_mod(N, K, D, D, p);
    var X3[100] = sm_sub_mod(N, K, F, D2, p);

    var dx[100] = sm_sub_mod(N, K, D, X3, p);
    var edx[100] = prod_mod_p(N, K, E, dx, p);
    var C2[100] = sm_add_mod(N, K, C, C, p);
    var C4[100] = sm_add_mod(N, K, C2, C2, p);
    var C8[100] = sm_add_mod(N, K, C4, C4, p);
    var Y3[100] = sm_sub_mod(N, K, edx, C8, p);

    var yz[100] = prod_mod_p(N, K, Y1, Z1, p);
    var Z3[100] = sm_add_mod(N, K, yz, yz, p);

    var out[3][100];
    for (var i = 0; i < 100; i++) {
        out[0][i] = 0;
        out[1][i] = 0;
        out[2][i] = 0;
    }
    for (var i = 0; i < K; i++) {
        out[0][i] = X3[i];
        out[1][i] = Y3[i];
        out[2][i] = Z3[i];
    }
    return out;
}

// Jacobian point plus affine point. The second point is (x2, y2) with an
// implied Z of 1, which is always the case here: the same base point is added
// at every set bit.
//
// H is the difference of the x-coordinates and r the difference of the y's, so
// H == 0 means the two points share an x-coordinate. They are then either the
// same point, where r == 0 and the answer is the doubling, or negations of one
// another, where the answer is infinity.
// out[0] = X3, out[1] = Y3, out[2] = Z3
function sm_jac_madd(N, K, X1, Y1, Z1, x2, y2, p) {
    var out[3][100];
    for (var i = 0; i < 100; i++) {
        out[0][i] = 0;
        out[1][i] = 0;
        out[2][i] = 0;
    }

    var Z1Z1[100] = prod_mod_p(N, K, Z1, Z1, p);
    var U2[100] = prod_mod_p(N, K, x2, Z1Z1, p);
    var y2z[100] = prod_mod_p(N, K, y2, Z1, p);
    var S2[100] = prod_mod_p(N, K, y2z, Z1Z1, p);

    var H[100] = sm_sub_mod(N, K, U2, X1, p);
    var rHalf[100] = sm_sub_mod(N, K, S2, Y1, p);
    var r[100] = sm_add_mod(N, K, rHalf, rHalf, p);

    var hIsZero = sm_is_zero(K, H);
    if (hIsZero == 1) {
        var rIsZero = sm_is_zero(K, r);
        if (rIsZero == 1) {
            var dbl[3][100] = sm_jac_double(N, K, X1, Y1, Z1, p);
            return dbl;
        }
        // negations of one another: the sum is the point at infinity, which
        // this representation writes as Z == 0
        return out;
    }

    var HH[100] = prod_mod_p(N, K, H, H, p);
    var HH2[100] = sm_add_mod(N, K, HH, HH, p);
    var I[100] = sm_add_mod(N, K, HH2, HH2, p);
    var J[100] = prod_mod_p(N, K, H, I, p);
    var V[100] = prod_mod_p(N, K, X1, I, p);

    var rSq[100] = prod_mod_p(N, K, r, r, p);
    var rSqJ[100] = sm_sub_mod(N, K, rSq, J, p);
    var V2[100] = sm_add_mod(N, K, V, V, p);
    var X3[100] = sm_sub_mod(N, K, rSqJ, V2, p);

    var vx[100] = sm_sub_mod(N, K, V, X3, p);
    var rvx[100] = prod_mod_p(N, K, r, vx, p);
    var yj[100] = prod_mod_p(N, K, Y1, J, p);
    var yj2[100] = sm_add_mod(N, K, yj, yj, p);
    var Y3[100] = sm_sub_mod(N, K, rvx, yj2, p);

    var zh[100] = sm_add_mod(N, K, Z1, H, p);
    var zhSq[100] = prod_mod_p(N, K, zh, zh, p);
    var zPre[100] = sm_sub_mod(N, K, zhSq, Z1Z1, p);
    var Z3[100] = sm_sub_mod(N, K, zPre, HH, p);

    for (var i = 0; i < K; i++) {
        out[0][i] = X3[i];
        out[1][i] = Y3[i];
        out[2][i] = Z3[i];
    }
    return out;
}

// out[0] = x, out[1] = y, out[2][0] = 1 when the result is the point at infinity
function secp256k1_scalarmul_func(N, K, scalar, x, y) {
    var p[100] = get_secp256k1_prime(N, K);
    var px[100] = sm_reduce(N, K, x, p);
    var py[100] = sm_reduce(N, K, y, p);

    var accX[100]; var accY[100]; var accZ[100];
    for (var i = 0; i < 100; i++) {
        accX[i] = 0; accY[i] = 0; accZ[i] = 0;
    }

    for (var limb = K - 1; limb >= 0; limb--) {
        for (var bit = N - 1; bit >= 0; bit--) {
            var d[3][100] = sm_jac_double(N, K, accX, accY, accZ, p);
            accX = d[0]; accY = d[1]; accZ = d[2];

            var b = (scalar[limb] \ (1 << bit)) % 2;
            if (b == 1) {
                var atInfinity = sm_is_zero(K, accZ);
                if (atInfinity == 1) {
                    for (var i = 0; i < 100; i++) {
                        accX[i] = 0; accY[i] = 0; accZ[i] = 0;
                    }
                    for (var i = 0; i < K; i++) {
                        accX[i] = px[i]; accY[i] = py[i];
                    }
                    accZ[0] = 1;
                } else {
                    var a[3][100] = sm_jac_madd(N, K, accX, accY, accZ, px, py, p);
                    accX = a[0]; accY = a[1]; accZ = a[2];
                }
            }
        }
    }

    var out[3][100];
    for (var i = 0; i < 100; i++) {
        out[0][i] = 0;
        out[1][i] = 0;
        out[2][i] = 0;
    }

    var isInf = sm_is_zero(K, accZ);
    if (isInf == 1) {
        out[2][0] = 1;
        return out;
    }

    // back to affine: x = X / Z^2, y = Y / Z^3, one inversion for the whole
    // multiplication
    var zInv[100] = mod_inv(N, K, accZ, p);
    var zInv2[100] = prod_mod_p(N, K, zInv, zInv, p);
    var zInv3[100] = prod_mod_p(N, K, zInv2, zInv, p);
    var xAff[100] = prod_mod_p(N, K, accX, zInv2, p);
    var yAff[100] = prod_mod_p(N, K, accY, zInv3, p);
    for (var i = 0; i < K; i++) {
        out[0][i] = xAff[i];
        out[1][i] = yAff[i];
    }
    return out;
}

// R = [u1]G + [u2]Q
function ecdsa_R_func(N, K, u1, u2, qx, qy) {
    var out[2][100];
    var gx[100] = get_gx64();
    var gy[100] = get_gy64();

    var A[3][100] = secp256k1_scalarmul_func(N, K, u1, gx, gy);
    var B[3][100] = secp256k1_scalarmul_func(N, K, u2, qx, qy);

    if (A[2][0] == 1) { out[0] = B[0]; out[1] = B[1]; return out; }
    if (B[2][0] == 1) { out[0] = A[0]; out[1] = A[1]; return out; }

    var same = 1;
    for (var i = 0; i < K; i++) { if (A[0][i] != B[0][i]) { same = 0; } }
    if (same == 1) {
        var d[2][100] = secp256k1_double_func(N, K, A[0], A[1]);
        out[0] = d[0]; out[1] = d[1];
        return out;
    }
    var s[2][100] = secp256k1_addunequal_func(N, K, A[0], A[1], B[0], B[1]);
    out[0] = s[0]; out[1] = s[1];
    return out;
}
