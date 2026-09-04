pragma circom 2.2.3;

include "../bitwise.circom";

// zk.golf KeccakF1600 submission 5bdce6d5-90a2-4208-bee3-0924b20ccb3b:
// two XOR3 rows per column bit, then one XOR3 and one chi row per state bit.
// Input bits must already be boolean. Lane (x,y), bit z is 64*(x+5*y)+z.
template KeccakF1600() {
    signal input in[1600];
    signal output out[1600];
    var rotation[25] = [
        0, 1, 62, 28, 27,
        36, 44, 6, 55, 20,
        3, 10, 43, 25, 39,
        41, 45, 15, 21, 8,
        18, 2, 61, 56, 14
    ];
    var rc[24] = [
        0x0000000000000001, 0x0000000000008082, 0x800000000000808a,
        0x8000000080008000, 0x000000000000808b, 0x0000000080000001,
        0x8000000080008081, 0x8000000000008009, 0x000000000000008a,
        0x0000000000000088, 0x0000000080008009, 0x000000008000000a,
        0x000000008000808b, 0x800000000000008b, 0x8000000000008089,
        0x8000000000008003, 0x8000000000008002, 0x8000000000000080,
        0x000000000000800a, 0x800000008000000a, 0x8000000080008081,
        0x8000000000008080, 0x0000000080000001, 0x8000000080008008
    ];
    signal state[25][1600];
    signal permuted[24][1600];
    component parity3[24][5];
    component parity5[24][5];
    component theta[24];
    component chi[24];
    state[0] <== in;
    for (var r = 0; r < 24; r++) {
        for (var x = 0; x < 5; x++) {
            parity3[r][x] = Xor3Bits(64);
            parity5[r][x] = Xor3Bits(64);
            for (var z = 0; z < 64; z++) {
                parity3[r][x].a[z] <== state[r][64*x+z];
                parity3[r][x].b[z] <== state[r][64*(x+5)+z];
                parity3[r][x].c[z] <== state[r][64*(x+10)+z];
            }
            parity5[r][x].a <== parity3[r][x].out;
            for (var z = 0; z < 64; z++) {
                parity5[r][x].b[z] <== state[r][64*(x+15)+z];
                parity5[r][x].c[z] <== state[r][64*(x+20)+z];
            }
        }
        theta[r] = Xor3Bits(1600);
        theta[r].a <== state[r];
        for (var x = 0; x < 5; x++) {
            for (var y = 0; y < 5; y++) {
                for (var z = 0; z < 64; z++) {
                    theta[r].b[64*(x+5*y)+z] <== parity5[r][(x+4)%5].out[z];
                    theta[r].c[64*(x+5*y)+z] <== parity5[r][(x+1)%5].out[(z+63)%64];
                }
            }
        }
        // Rho rotates each lane; pi maps (x,y) to (y,2*x+3*y).
        for (var x = 0; x < 5; x++) {
            for (var y = 0; y < 5; y++) {
                for (var z = 0; z < 64; z++) {
                    permuted[r][64*(y+5*((2*x+3*y)%5))+z] <==
                        theta[r].out[64*(x+5*y)+(z+64-rotation[x+5*y])%64];
                }
            }
        }
        chi[r] = ChiBits(1600);
        chi[r].a <== permuted[r];
        for (var x = 0; x < 5; x++) {
            for (var y = 0; y < 5; y++) {
                for (var z = 0; z < 64; z++) {
                    chi[r].b[64*(x+5*y)+z] <== permuted[r][64*((x+1)%5+5*y)+z];
                    chi[r].c[64*(x+5*y)+z] <== permuted[r][64*((x+2)%5+5*y)+z];
                }
            }
        }
        for (var i = 0; i < 1600; i++) {
            if (i < 64 && ((rc[r] >> i) & 1)) {
                state[r+1][i] <== 1 - chi[r].out[i];
            } else {
                state[r+1][i] <== chi[r].out[i];
            }
        }
    }
    out <== state[24];
}
