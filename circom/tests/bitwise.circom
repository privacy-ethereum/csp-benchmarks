pragma circom 2.2.3;
include "../circuits/bitwise.circom";

template CheckBitwise() {
    signal input bits[3];
    signal output out[3];
    component parity = Xor3Bits(1);
    component majority = MajorityBits(1);
    component chi = ChiBits(1);
    for (var i = 0; i < 3; i++) bits[i] * (bits[i] - 1) === 0;
    parity.a[0] <== bits[0];
    parity.b[0] <== bits[1];
    parity.c[0] <== bits[2];
    majority.a[0] <== bits[0];
    majority.b[0] <== bits[1];
    majority.c[0] <== bits[2];
    chi.a[0] <== bits[0];
    chi.b[0] <== bits[1];
    chi.c[0] <== bits[2];
    out[0] <== parity.out[0];
    out[1] <== majority.out[0];
    out[2] <== chi.out[0];
}
component main = CheckBitwise();
