#![no_std]

pub const PRIVATE_TX_PUBLIC_OUTPUT_BYTES: usize = 32;

pub fn evaluate_private_tx(input: &[u8]) -> [u8; PRIVATE_TX_PUBLIC_OUTPUT_BYTES] {
    let mut offset = 0usize;
    let depth = read_u32(input, &mut offset) as usize;
    assert!(depth <= u64::BITS as usize);

    let mut input_owners = [0u64; 2];
    let mut input_amounts = [0i64; 2];
    for i in 0..2 {
        input_owners[i] = read_u64(input, &mut offset);
        input_amounts[i] = read_i64(input, &mut offset);
    }

    let mut output_owners = [0u64; 2];
    let mut output_amounts = [0i64; 2];
    for i in 0..2 {
        output_owners[i] = read_u64(input, &mut offset);
        output_amounts[i] = read_i64(input, &mut offset);
    }

    for amount in input_amounts.iter().chain(output_amounts.iter()) {
        assert!(*amount >= 0);
    }
    assert_eq!(
        checked_add(input_amounts[0], input_amounts[1]),
        checked_add(output_amounts[0], output_amounts[1])
    );

    let mut roots = [0i64; 2];
    for i in 0..2 {
        let expected_root = read_i64(input, &mut offset);
        let path_index = read_u64(input, &mut offset);
        let mut acc = note_commitment(input_owners[i], input_amounts[i]);
        for level in 0..depth {
            let sibling = read_i64(input, &mut offset);
            acc = if ((path_index >> level) & 1) == 1 {
                checked_add(sibling, acc)
            } else {
                checked_add(acc, sibling)
            };
        }
        assert_eq!(acc, expected_root);
        roots[i] = acc;
    }
    assert_eq!(offset, input.len());

    let mut output = [0u8; PRIVATE_TX_PUBLIC_OUTPUT_BYTES];
    write_i64_at(&mut output, 0, roots[0]);
    write_i64_at(&mut output, 8, roots[1]);
    write_i64_at(
        &mut output,
        16,
        note_commitment(output_owners[0], output_amounts[0]),
    );
    write_i64_at(
        &mut output,
        24,
        note_commitment(output_owners[1], output_amounts[1]),
    );
    output
}

fn note_commitment(owner: u64, amount: i64) -> i64 {
    let owner = i64::try_from(owner).expect("owner does not fit in i64");
    checked_add(owner, amount)
}

fn checked_add(lhs: i64, rhs: i64) -> i64 {
    lhs.checked_add(rhs).expect("private_tx addition overflow")
}

fn read_u32(input: &[u8], offset: &mut usize) -> u32 {
    let mut bytes = [0u8; 4];
    bytes.copy_from_slice(read_exact(input, offset, 4));
    u32::from_le_bytes(bytes)
}

fn read_u64(input: &[u8], offset: &mut usize) -> u64 {
    let mut bytes = [0u8; 8];
    bytes.copy_from_slice(read_exact(input, offset, 8));
    u64::from_le_bytes(bytes)
}

fn read_i64(input: &[u8], offset: &mut usize) -> i64 {
    let mut bytes = [0u8; 8];
    bytes.copy_from_slice(read_exact(input, offset, 8));
    i64::from_le_bytes(bytes)
}

fn read_exact<'a>(input: &'a [u8], offset: &mut usize, len: usize) -> &'a [u8] {
    let end = (*offset).checked_add(len).expect("input offset overflow");
    let bytes = input.get(*offset..end).expect("truncated private_tx input");
    *offset = end;
    bytes
}

fn write_i64_at(output: &mut [u8; PRIVATE_TX_PUBLIC_OUTPUT_BYTES], offset: usize, value: i64) {
    output[offset..offset + 8].copy_from_slice(&value.to_le_bytes());
}
