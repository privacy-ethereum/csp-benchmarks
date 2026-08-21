/*
    ECDSA verification on secp256k1.

    R == [u1]G + [u2]Q is rearranged as [u2]Q == R - [u1]G: the right-hand side
    is computed with a fixed-base comb, the left-hand side is verified with the
    4-dimensional fake-GLV circuit. One variable-base multiplication instead of
    two.

    R is not witnessed as a point: R.x == r and r is public, so only y is
    witnessed and one on-curve check covers it. The sign of y is pinned by the
    verification equation, since -R would require R of order 2.

    Semantics follow k256's verify_prehash, which the other backends in this
    repository prove: the public key is validated and r, s must lie in
    [1, n-1]. circom-ecdsa's ECDSAVerifyNoPubkeyCheck does neither, so its
    constraint count is not a like-for-like comparison. There is no `result`
    output; an invalid signature fails witness generation.

    Known deviation, shared with circom-ecdsa: the standard compares R.x mod n
    against r, here R.x == r directly, so signatures with R.x >= n are rejected
    (probability ~2^-128).
*/
pragma circom 2.0.2;

// ./ecdsa.circom is deliberately not included: the only thing used from it was
// ECDSAPrivToPub, which drags in ecdsa_func.circom (3.19 MB of stride-8 table).
include "./comb_fixed.circom";
include "./glv4_scalarmul.circom";
include "./eisenstein_func.circom";
include "./scalarmul_func.circom";
include "../../circomlib/circuits/comparators.circom";

template ECDSA4CombVerify() {
    signal input r[4];
    signal input s[4];
    signal input msghash[4];
    signal input pubkey[2][4];

    // Witness-only values: the inverse of s, the y coordinate of R, and the
    // lattice hint. They used to be private inputs, which made this circuit's
    // interface differ from the other ECDSA circuits here; computing them below
    // leaves the input to the public part alone. None of the constraints change:
    // they never trusted these values, they check them.
    signal sinv[4];
    signal Ry[4];
    signal mag[4];
    signal sgn[4];

    var ordN[100] = get_secp256k1_order(64, 4);
    var prime[100] = get_secp256k1_prime(64, 4);

    signal ordSig[4];
    signal primeSig[4];
    for (var j = 0; j < 4; j++) {
        ordSig[j] <== ordN[j];
        primeSig[j] <== prime[j];
    }

    // ---------- 1. validation: r, s in [1, n-1] ----------
    // Four limbs below 2^64 sum to less than 2^66, so the sum fits in the
    // BN254 field and is zero exactly when every limb is zero.
    component rLtN = BigLessThan(64, 4);
    component sLtN = BigLessThan(64, 4);
    for (var j = 0; j < 4; j++) {
        rLtN.a[j] <== r[j];  rLtN.b[j] <== ordSig[j];
        sLtN.a[j] <== s[j];  sLtN.b[j] <== ordSig[j];
    }
    rLtN.out === 1;
    sLtN.out === 1;

    component rZero = IsZero();
    component sZero = IsZero();
    rZero.in <== r[0] + r[1] + r[2] + r[3];
    sZero.in <== s[0] + s[1] + s[2] + s[3];
    rZero.out === 0;
    sZero.out === 0;

    // ---------- 2. the public key is on the curve ----------
    component qOn = Secp256k1PointOnCurve();
    for (var j = 0; j < 4; j++) {
        qOn.x[j] <== pubkey[0][j];
        qOn.y[j] <== pubkey[1][j];
    }

    // ---------- 3. sinv computed, checked with one multiplication ----------
    // s is nonzero and below n, and n is prime, so the inverse exists.
    var sv[100];
    var ov[100];
    for (var j = 0; j < 100; j++) {
        sv[j] = 0;
        ov[j] = 0;
    }
    for (var j = 0; j < 4; j++) {
        sv[j] = s[j];
        ov[j] = ordN[j];
    }
    var sinvVal[100] = mod_inv(64, 4, sv, ov);
    for (var j = 0; j < 4; j++) {
        sinv[j] <-- sinvVal[j];
    }

    // The range check is mandatory, otherwise the limbs may exceed 64 bits.
    component sinvRange[4];
    for (var j = 0; j < 4; j++) {
        sinvRange[j] = Num2Bits(64);
        sinvRange[j].in <== sinv[j];
    }
    component sinvCheck = BigMultModP(64, 4);
    for (var j = 0; j < 4; j++) {
        sinvCheck.a[j] <== sinv[j];
        sinvCheck.b[j] <== s[j];
        sinvCheck.p[j] <== ordSig[j];
    }
    sinvCheck.out[0] === 1;
    for (var j = 1; j < 4; j++) {
        sinvCheck.out[j] === 0;
    }

    // ---------- 4. u1 = h*sinv, u2 = r*sinv (mod n) ----------
    component u1c = BigMultModP(64, 4);
    component u2c = BigMultModP(64, 4);
    for (var j = 0; j < 4; j++) {
        u1c.a[j] <== msghash[j]; u1c.b[j] <== sinv[j]; u1c.p[j] <== ordSig[j];
        u2c.a[j] <== r[j];       u2c.b[j] <== sinv[j]; u2c.p[j] <== ordSig[j];
    }

    // R = [u1]G + [u2]Q, computed off-constraint so that only its y coordinate
    // has to be carried: x is r, which is public. The result is checked, not
    // trusted, by the on-curve test below and by the verification equation.
    var u1v[100];
    var u2v[100];
    var qxv[100];
    var qyv[100];
    for (var j = 0; j < 100; j++) {
        u1v[j] = 0;
        u2v[j] = 0;
        qxv[j] = 0;
        qyv[j] = 0;
    }
    for (var j = 0; j < 4; j++) {
        u1v[j] = u1c.out[j];
        u2v[j] = u2c.out[j];
        qxv[j] = pubkey[0][j];
        qyv[j] = pubkey[1][j];
    }
    var Rval[2][100] = ecdsa_R_func(64, 4, u1v, u2v, qxv, qyv);
    for (var j = 0; j < 4; j++) {
        Ry[j] <-- Rval[1][j];
    }

    // ---------- 5. R = (r, Ry) ----------
    // Ry needs a range check and canonicity (Ry < p): Secp256k1PointOnCurve
    // checks x^3 + 7 - y^2 == 0 mod p, which does not force y < p.
    component ryRange[4];
    for (var j = 0; j < 4; j++) {
        ryRange[j] = Num2Bits(64);
        ryRange[j].in <== Ry[j];
    }
    component ryLtP = BigLessThan(64, 4);
    for (var j = 0; j < 4; j++) {
        ryLtP.a[j] <== Ry[j];
        ryLtP.b[j] <== primeSig[j];
    }
    ryLtP.out === 1;

    component rOn = Secp256k1PointOnCurve();
    for (var j = 0; j < 4; j++) {
        rOn.x[j] <== r[j];        // r < n < p, so r is a valid field element
        rOn.y[j] <== Ry[j];
    }

    // ---------- 6. [u1]G, fixed base, via the width-12 comb ----------
    component u1G = CombFixedBase();
    for (var j = 0; j < 4; j++) {
        u1G.k[j] <== u1c.out[j];
    }

    // ---------- 7a. the subtraction must not be degenerate ----------
    // Load-bearing, not caution: Secp256k1AddUnequal leaves its output
    // unconstrained when the operands coincide (see Secp256k1AddStrict in
    // glv4_straus.circom). A prover who supplies r = ([u1]G).x and
    // Ry = p - ([u1]G).y gets a free S, sets it equal to [u2]Q for an
    // arbitrary Q, and verifies without the private key.
    //
    // Equality of limbs means equality of values only for canonical
    // representations: [u1]G leaves the table through a one-hot selector and
    // is canonical by construction, r is public and constrained here.
    component rRange[4];
    for (var j = 0; j < 4; j++) {
        rRange[j] = Num2Bits(64);
        rRange[j].in <== r[j];
    }
    component xSame[4];
    signal xSameAcc[4];
    for (var j = 0; j < 4; j++) {
        xSame[j] = IsZero();
        xSame[j].in <== r[j] - u1G.out[0][j];
    }
    xSameAcc[0] <== xSame[0].out;
    for (var j = 1; j < 4; j++) xSameAcc[j] <== xSameAcc[j - 1] * xSame[j].out;
    xSameAcc[3] === 0;

    // ---------- 7. S = R - [u1]G ----------
    component negU1Gy = BigSub(64, 4);
    for (var j = 0; j < 4; j++) {
        negU1Gy.a[j] <== primeSig[j];
        negU1Gy.b[j] <== u1G.out[1][j];
    }

    component Ssub = Secp256k1AddUnequal(64, 4);
    for (var j = 0; j < 4; j++) {
        Ssub.a[0][j] <== r[j];
        Ssub.a[1][j] <== Ry[j];
        Ssub.b[0][j] <== u1G.out[0][j];
        Ssub.b[1][j] <== negU1Gy.out[j];
    }

    // The lattice hint for u2. GLV4ScalarMulVerify constrains it fully, so a
    // wrong hint cannot pass; computing it here only spares the caller the
    // lattice reduction.
    var hint[2][4] = fake_glv_decompose(64, 12, u2v);
    for (var i = 0; i < 4; i++) {
        mag[i] <-- hint[0][i];
        sgn[i] <-- hint[1][i];
    }

    // ---------- 8. [u2]Q == S, via 4-dimensional fake-GLV ----------
    // Brings its own u2 < n check, the fully constrained hint, the
    // endomorphisms, the Straus loop and the terminal =O assertion.
    component glv = GLV4ScalarMulVerify();
    for (var j = 0; j < 4; j++) {
        glv.scalar[j] <== u2c.out[j];
        glv.P[0][j] <== pubkey[0][j];
        glv.P[1][j] <== pubkey[1][j];
        glv.Q[0][j] <== Ssub.out[0][j];
        glv.Q[1][j] <== Ssub.out[1][j];
    }
    for (var i = 0; i < 4; i++) {
        glv.mag[i] <== mag[i];
        glv.sgn[i] <== sgn[i];
    }
}
