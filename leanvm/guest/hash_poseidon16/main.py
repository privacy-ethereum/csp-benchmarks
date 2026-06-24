HASH_COUNT = HASH_COUNT_PLACEHOLDER
DIGEST_LEN = 8
INPUT_LEN = 16


def main():
    inputs = Array(HASH_COUNT * INPUT_LEN)
    hint_witness("poseidon16_inputs", inputs)

    outputs = Array(HASH_COUNT * DIGEST_LEN)
    folds = Array((HASH_COUNT + 1) * DIGEST_LEN)

    for cell in unroll(0, DIGEST_LEN):
        folds[cell] = 0

    for i in range(0, HASH_COUNT):
        input_offset = i * INPUT_LEN
        output_offset = i * DIGEST_LEN
        poseidon16_compress_half(
            inputs + input_offset,
            inputs + input_offset + DIGEST_LEN,
            outputs + output_offset,
        )

        prev = i * DIGEST_LEN
        next = (i + 1) * DIGEST_LEN
        for cell in unroll(0, DIGEST_LEN):
            folds[next + cell] = folds[prev + cell] + outputs[output_offset + cell]

    public = 0
    final_offset = HASH_COUNT * DIGEST_LEN
    for cell in unroll(0, DIGEST_LEN):
        public[cell] = folds[final_offset + cell]
    return
