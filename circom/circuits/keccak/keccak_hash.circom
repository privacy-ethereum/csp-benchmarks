pragma circom 2.0.0;

include "../../hash-circuits/circuits/keccak/keccak_bytes.circom";

template Keccak256Hash(N) {
    signal input in[N];
    signal input hash[32];
    signal output out[32];

    component keccak = Keccak_256_bytes(N);
    keccak.inp_bytes <== in;
    out <== keccak.out_bytes;

    for (var i = 0; i < 32; i++) {
        out[i] === hash[i];
    }
}
