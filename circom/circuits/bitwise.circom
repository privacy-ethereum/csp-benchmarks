pragma circom 2.2.3;

// Single-row identities from the verified zk.golf submissions:
// SHA256: 75f88d68-6cae-4acf-8adc-7d41bce17042 (Xor3, Maj32).
// KeccakF1600: 5bdce6d5-90a2-4208-bee3-0924b20ccb3b (Xor3Lane, ChiLane).
// Callers must constrain every input to a bit. Each row then has a unique
// output, which is also a bit; no additional output booleanity is needed.

template Xor3Bits(N) {
    signal input a[N];
    signal input b[N];
    signal input c[N];
    signal output out[N];
    for (var i = 0; i < N; i++) {
        out[i] <-- a[i] ^ b[i] ^ c[i];
        // The coefficient of out is in {-3,-2,-1,1,2,3}.
        (out[i] + 2*a[i] + 2*b[i] + 7*c[i]) *
            (a[i] + b[i] - 4*c[i] + 1) === 6*a[i] + 6*b[i] - 24*c[i];
    }
}

template MajorityBits(N) {
    signal input a[N];
    signal input b[N];
    signal input c[N];
    signal output out[N];
    for (var i = 0; i < N; i++) {
        out[i] <-- (a[i] & b[i]) ^ (a[i] & c[i]) ^ (b[i] & c[i]);
        // The coefficient of out is in {-4,-3,-2,2,3,4}.
        (out[i] + a[i] + b[i] - 9*c[i] + 3) *
            (a[i] + b[i] + 6*c[i] - 4) === -12;
    }
}

template ChiBits(N) {
    signal input a[N];
    signal input b[N];
    signal input c[N];
    signal output out[N];
    for (var i = 0; i < N; i++) {
        out[i] <-- a[i] ^ ((1 - b[i]) & c[i]);
        // The coefficient of out is in {-3,-2,-1,1,2,3}.
        (out[i] + 3*a[i] - b[i] - c[i]) *
            (4*a[i] + b[i] + c[i] - 3) === 4*a[i] + 2*b[i];
    }
}
