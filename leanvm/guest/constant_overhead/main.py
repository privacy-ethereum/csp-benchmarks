def main():
    witness = Array(1)
    hint_witness("constant_overhead", witness)
    result = witness[0] + 2
    assert result == 4

    public = 0
    public[0] = result
    return
