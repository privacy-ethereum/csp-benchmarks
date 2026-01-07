pragma circom 2.0.0;

include "../../circomlib/circuits/poseidon.circom";

template PoseidonHash(N) {
    signal input inputs[N];
    signal output out;

    component poseidon = Poseidon(N);
    for (var i = 0; i < N; i++) {
        poseidon.inputs[i] <== inputs[i];
    }
    out <== poseidon.out;
}
