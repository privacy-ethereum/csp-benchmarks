BRANCH_COUNT = BRANCH_COUNT_PLACEHOLDER
DEPTH = 32
DIGEST_LIMBS = 16
PUBLIC_CELLS = 8
BRANCH_CELLS = DIGEST_LIMBS + DEPTH + DEPTH * DIGEST_LIMBS


def main():
    branches = Array(BRANCH_COUNT * BRANCH_CELLS)
    hint_witness("merkle_blake3", branches)

    nodes = Array(BRANCH_COUNT * (DEPTH + 1) * DIGEST_LIMBS)
    folds = Array((BRANCH_COUNT + 1) * PUBLIC_CELLS)

    for cell in unroll(0, PUBLIC_CELLS):
        folds[cell] = 0

    for branch in range(0, BRANCH_COUNT):
        branch_base = branch * BRANCH_CELLS
        node_base = branch * (DEPTH + 1) * DIGEST_LIMBS

        for cell in unroll(0, DIGEST_LIMBS):
            nodes[node_base + cell] = branches[branch_base + cell]

        for level in unroll(0, DEPTH):
            bit = branches[branch_base + DIGEST_LIMBS + level]
            assert bit * (bit - 1) == 0
            current = nodes + node_base + level * DIGEST_LIMBS
            next = nodes + node_base + (level + 1) * DIGEST_LIMBS
            sibling = branches + branch_base + DIGEST_LIMBS + DEPTH + level * DIGEST_LIMBS
            if bit == 1:
                blake3_hash_64(sibling, current, next)
            else:
                blake3_hash_64(current, sibling, next)

        root = nodes + node_base + DEPTH * DIGEST_LIMBS
        prev_fold = branch * PUBLIC_CELLS
        next_fold = (branch + 1) * PUBLIC_CELLS
        for cell in unroll(0, PUBLIC_CELLS):
            folds[next_fold + cell] = (
                folds[prev_fold + cell]
                + root[cell]
                + root[PUBLIC_CELLS + cell]
            )

    public = 0
    final_offset = BRANCH_COUNT * PUBLIC_CELLS
    for cell in unroll(0, PUBLIC_CELLS):
        public[cell] = folds[final_offset + cell]
    return
