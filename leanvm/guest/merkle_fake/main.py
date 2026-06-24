BRANCH_COUNT = BRANCH_COUNT_PLACEHOLDER
DEPTH = 32
MAX_AMOUNT = 65535


@inline
def assert_amount(amount):
    assert amount <= MAX_AMOUNT
    return


@inline
def fake_amount(branch: Const):
    group = div_floor(branch, 4)
    rem = branch % 4
    if rem == 0:
        return 70 + group
    elif rem == 1:
        return 50 + group
    elif rem == 2:
        return 80 + group
    else:
        return 40 + group


@inline
def fake_owner(branch: Const):
    return 10000 + branch


@inline
def fake_sibling(branch: Const, level: Const):
    return 1 + ((branch * 97 + level * 131) % 10000)


def main():
    bits = Array(BRANCH_COUNT * DEPTH)
    hint_witness("merkle_fake_bits", bits)

    fold0: Mut = 0
    fold1: Mut = 0
    fold2: Mut = 0
    fold3: Mut = 0

    group_inputs: Mut = 0
    group_outputs: Mut = 0

    for branch in unroll(0, BRANCH_COUNT):
        amount = fake_amount(branch)
        assert_amount(amount)
        acc: Mut = fake_owner(branch) + amount

        rem = branch % 4
        if rem == 0:
            group_inputs += amount
        elif rem == 1:
            group_inputs += amount
        elif rem == 2:
            group_outputs += amount
        else:
            group_outputs += amount

        if rem == 3:
            assert group_inputs == group_outputs
            group_inputs = 0
            group_outputs = 0

        for level in unroll(0, DEPTH):
            bit = bits[branch * DEPTH + level]
            assert bit * (bit - 1) == 0
            sibling = fake_sibling(branch, level)
            if bit == 1:
                acc = sibling + acc
            else:
                acc = acc + sibling

        if rem == 0:
            fold0 += acc
        elif rem == 1:
            fold1 += acc
        elif rem == 2:
            fold2 += acc
        else:
            fold3 += acc

    public = 0
    public[0] = fold0
    public[1] = fold1
    public[2] = fold2
    public[3] = fold3
    return
