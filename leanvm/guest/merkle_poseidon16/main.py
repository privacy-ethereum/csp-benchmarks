BRANCH_COUNT = BRANCH_COUNT_PLACEHOLDER
DEPTH = 32
DIGEST_LEN = 8
BRANCH_CELLS = DIGEST_LEN + DEPTH + DEPTH * DIGEST_LEN


def main():
    branches = Array(BRANCH_COUNT * BRANCH_CELLS)
    hint_witness("merkle_poseidon16", branches)

    nodes = Array(BRANCH_COUNT * (DEPTH + 1) * DIGEST_LEN)
    folds = Array((BRANCH_COUNT + 1) * DIGEST_LEN)

    for cell in unroll(0, DIGEST_LEN):
        folds[cell] = 0

    for branch in range(0, BRANCH_COUNT):
        branch_base = branch * BRANCH_CELLS
        node_base = branch * (DEPTH + 1) * DIGEST_LEN

        for cell in unroll(0, DIGEST_LEN):
            nodes[node_base + cell] = branches[branch_base + cell]

        for level in unroll(0, DEPTH):
            bit = branches[branch_base + DIGEST_LEN + level]
            assert bit * (bit - 1) == 0
            current = nodes + node_base + level * DIGEST_LEN
            next = nodes + node_base + (level + 1) * DIGEST_LEN
            sibling = branches + branch_base + DIGEST_LEN + DEPTH + level * DIGEST_LEN
            if bit == 1:
                poseidon16_compress_half(sibling, current, next)
            else:
                poseidon16_compress_half(current, sibling, next)

        root = nodes + node_base + DEPTH * DIGEST_LEN
        prev_fold = branch * DIGEST_LEN
        next_fold = (branch + 1) * DIGEST_LEN
        for cell in unroll(0, DIGEST_LEN):
            folds[next_fold + cell] = folds[prev_fold + cell] + root[cell]

    public = 0
    final_offset = BRANCH_COUNT * DIGEST_LEN
    for cell in unroll(0, DIGEST_LEN):
        public[cell] = folds[final_offset + cell]
    return
