pragma circom 2.2.3;

include "../bitwise.circom";
include "../../circomlib/circuits/bitify.circom";
include "../../circomlib/circuits/sha256/sha256compression_function.circom";

// Little-endian bits within each word. Inputs must already be boolean.
template Sha256Sigma(R0, R1, R2, SHIFT) {
    signal input in[32];
    signal output out[32];
    component parity = Xor3Bits(32);
    for (var i = 0; i < 32; i++) {
        parity.a[i] <== in[(i + R0) % 32];
        parity.b[i] <== in[(i + R1) % 32];
        if (SHIFT && i + R2 >= 32) {
            parity.c[i] <== 0;
        } else {
            parity.c[i] <== in[(i + R2) % 32];
        }
    }
    out <== parity.out;
}

template Sha256Compression() {
    signal input block[16][32];
    signal input state[8][32];
    signal output out[8][32];
    signal w[64][32];
    signal s[65][8][32];
    component scheduleSigma0[48];
    component scheduleSigma1[48];
    component scheduleSum[48];
    for (var t = 0; t < 64; t++) {
        if (t < 16) {
            w[t] <== block[t];
        } else {
            scheduleSigma0[t-16] = Sha256Sigma(7, 18, 3, 1);
            scheduleSigma1[t-16] = Sha256Sigma(17, 19, 10, 1);
            scheduleSigma0[t-16].in <== w[t-15];
            scheduleSigma1[t-16].in <== w[t-2];
            var sum = 0;
            for (var j = 0; j < 32; j++) {
                sum += (scheduleSigma0[t-16].out[j] + scheduleSigma1[t-16].out[j]
                    + w[t-7][j] + w[t-16][j]) * 2**j;
            }
            // Four 32-bit words sum to less than 2^34. Constrain the carry
            // together with the low word so reduction cannot wrap in the field.
            scheduleSum[t-16] = Num2Bits(34);
            scheduleSum[t-16].in <== sum;
            for (var j = 0; j < 32; j++) w[t][j] <== scheduleSum[t-16].out[j];
        }
    }

    s[0] <== state;
    component sigma0[64];
    component sigma1[64];
    component majority[64];
    component sumE[64];
    component sumA[64];
    signal choice[64][32];
    for (var t = 0; t < 64; t++) {
        sigma0[t] = Sha256Sigma(2, 13, 22, 0);
        sigma1[t] = Sha256Sigma(6, 11, 25, 0);
        majority[t] = MajorityBits(32);
        sigma0[t].in <== s[t][0];
        sigma1[t].in <== s[t][4];
        majority[t].a <== s[t][0];
        majority[t].b <== s[t][1];
        majority[t].c <== s[t][2];
        var eSum = sha256K(t);
        for (var j = 0; j < 32; j++) {
            choice[t][j] <== s[t][4][j] * (s[t][5][j] - s[t][6][j]) + s[t][6][j];
            eSum += (s[t][3][j] + s[t][7][j] + sigma1[t].out[j]
                + choice[t][j] + w[t][j]) * 2**j;
        }
        // Following the submission's SHA256Round: reduce d + T1 once,
        // then reuse new_e to obtain new_a = new_e - d + Sigma0(a) + Maj.
        sumE[t] = Num2Bits(35);
        sumE[t].in <== eSum;
        var aSum = 1;
        for (var j = 0; j < 32; j++) {
            aSum += (sumE[t].out[j] + (1 - s[t][3][j])
                + sigma0[t].out[j] + majority[t].out[j]) * 2**j;
        }
        sumA[t] = Num2Bits(34);
        sumA[t].in <== aSum;
        for (var j = 0; j < 32; j++) {
            s[t+1][0][j] <== sumA[t].out[j];
            s[t+1][4][j] <== sumE[t].out[j];
        }
        for (var lane = 1; lane < 8; lane++) {
            if (lane != 4) s[t+1][lane] <== s[t][lane-1];
        }
    }

    component feedForward[8];
    for (var lane = 0; lane < 8; lane++) {
        var sum = 0;
        for (var j = 0; j < 32; j++) sum += (state[lane][j] + s[64][lane][j]) * 2**j;
        feedForward[lane] = Num2Bits(33);
        feedForward[lane].in <== sum;
        for (var j = 0; j < 32; j++) out[lane][j] <== feedForward[lane].out[j];
    }
}
