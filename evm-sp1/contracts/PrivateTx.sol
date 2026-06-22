// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

contract PrivateTx {
    uint256 internal constant DEPTH = 32;

    function verifyPrivateTx(
        uint64[4] calldata owners,
        int64[4] calldata amounts,
        int64[2] calldata expectedRoots,
        uint64[2] calldata pathIndices,
        int64[64] calldata siblings
    ) external pure returns (int64[4] memory publicOutput) {
        require(amounts[0] >= 0 && amounts[1] >= 0 && amounts[2] >= 0 && amounts[3] >= 0);
        require(amounts[0] + amounts[1] == amounts[2] + amounts[3]);

        int64 root0 = noteCommitment(owners[0], amounts[0]);
        int64 root1 = noteCommitment(owners[1], amounts[1]);
        for (uint256 i = 0; i < DEPTH; i++) {
            if (((pathIndices[0] >> i) & uint64(1)) == 1) {
                root0 = pseudoHash(siblings[i], root0);
            } else {
                root0 = pseudoHash(root0, siblings[i]);
            }
            if (((pathIndices[1] >> i) & uint64(1)) == 1) {
                root1 = pseudoHash(siblings[DEPTH + i], root1);
            } else {
                root1 = pseudoHash(root1, siblings[DEPTH + i]);
            }
        }
        require(root0 == expectedRoots[0]);
        require(root1 == expectedRoots[1]);

        publicOutput[0] = root0;
        publicOutput[1] = root1;
        publicOutput[2] = noteCommitment(owners[2], amounts[2]);
        publicOutput[3] = noteCommitment(owners[3], amounts[3]);
    }

    function noteCommitment(uint64 owner, int64 amount) internal pure returns (int64) {
        return int64(uint64(owner)) + amount;
    }

    function pseudoHash(int64 left, int64 right) internal pure returns (int64) {
        return left + right;
    }
}
