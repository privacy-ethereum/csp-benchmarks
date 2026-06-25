HASH_COUNT = HASH_COUNT_PLACEHOLDER
DIGEST_LIMBS = 16
INPUT_LIMBS = 32
INPUT_HALF = 16
PUBLIC_CELLS = 8


def main():
    inputs = Array(HASH_COUNT * INPUT_LIMBS)
    hint_witness("blake3_inputs", inputs)

    outputs = Array(HASH_COUNT * DIGEST_LIMBS)
    folds = Array((HASH_COUNT + 1) * PUBLIC_CELLS)

    for cell in unroll(0, PUBLIC_CELLS):
        folds[cell] = 0

    for i in range(0, HASH_COUNT):
        input_offset = i * INPUT_LIMBS
        output_offset = i * DIGEST_LIMBS
        blake3_hash_64(
            inputs + input_offset,
            inputs + input_offset + INPUT_HALF,
            outputs + output_offset,
        )

        prev = i * PUBLIC_CELLS
        next = (i + 1) * PUBLIC_CELLS
        for cell in unroll(0, PUBLIC_CELLS):
            folds[next + cell] = (
                folds[prev + cell]
                + outputs[output_offset + cell]
                + outputs[output_offset + PUBLIC_CELLS + cell]
            )

    public = 0
    final_offset = HASH_COUNT * PUBLIC_CELLS
    for cell in unroll(0, PUBLIC_CELLS):
        public[cell] = folds[final_offset + cell]
    return
