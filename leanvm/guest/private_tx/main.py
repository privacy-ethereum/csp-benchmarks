DEPTH = 32
NOTE_CELLS = 8
BRANCH_CELLS = 1 + DEPTH + DEPTH
TX_CELLS = NOTE_CELLS + 2 * BRANCH_CELLS
# LeanVM range checks support bounds below 2^16. This exact bound also rejects
# negative host i64 amounts after field conversion.
MAX_AMOUNT = 65535


@inline
def note_commitment(owner, amount):
    return owner + amount


@inline
def assert_amount(amount):
    assert amount <= MAX_AMOUNT
    return


@inline
def assert_equal(left, right):
    # Equality over compound expressions is represented with write-once memory:
    # the second write is valid only if it matches the first write.
    equality = Array(1)
    equality[0] = left
    equality[0] = right
    return


@inline
def verify_branch(leaf, branch):
    acc: Mut = leaf
    for level in unroll(0, DEPTH):
        bit = branch[1 + level]
        sibling = branch[1 + DEPTH + level]
        assert bit * (bit - 1) == 0
        if bit == 1:
            acc = sibling + acc
        else:
            acc = acc + sibling
    expected_root = branch[0]
    assert acc == expected_root
    return acc


def main():
    tx = Array(TX_CELLS)
    hint_witness("private_tx", tx)

    input_owner0 = tx[0]
    input_amount0 = tx[1]
    input_owner1 = tx[2]
    input_amount1 = tx[3]
    output_owner0 = tx[4]
    output_amount0 = tx[5]
    output_owner1 = tx[6]
    output_amount1 = tx[7]

    assert_amount(input_amount0)
    assert_amount(input_amount1)
    assert_amount(output_amount0)
    assert_amount(output_amount1)

    assert_equal(
        input_amount0 + input_amount1,
        output_amount0 + output_amount1,
    )

    input_commitment0 = note_commitment(input_owner0, input_amount0)
    input_commitment1 = note_commitment(input_owner1, input_amount1)
    output_commitment0 = note_commitment(output_owner0, output_amount0)
    output_commitment1 = note_commitment(output_owner1, output_amount1)

    branch0 = tx + NOTE_CELLS
    branch1 = branch0 + BRANCH_CELLS
    root0 = verify_branch(input_commitment0, branch0)
    root1 = verify_branch(input_commitment1, branch1)

    # Address 0 points at LeanVM's public-input memory. These writes bind the
    # computed outputs to the verifier-supplied public cells.
    public = 0
    public[0] = root0
    public[1] = root1
    public[2] = output_commitment0
    public[3] = output_commitment1
    return
