use rand::{Rng, SeedableRng, rngs::StdRng};
use std::error::Error;
use std::fmt;

pub const PRIVATE_TX_INPUT_COUNT: usize = 2;
pub const PRIVATE_TX_OUTPUT_COUNT: usize = 2;
pub const PRIVATE_TX_PUBLIC_OUTPUT_BYTES: usize = 32;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PrivateTxNote {
    pub owner: u64,
    pub amount: i64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PrivateTxMerkleBranch {
    pub expected_root: i64,
    pub path_index: u64,
    pub siblings: Vec<i64>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PrivateTxInput {
    pub depth: usize,
    pub input_notes: [PrivateTxNote; PRIVATE_TX_INPUT_COUNT],
    pub output_notes: [PrivateTxNote; PRIVATE_TX_OUTPUT_COUNT],
    pub input_branches: [PrivateTxMerkleBranch; PRIVATE_TX_INPUT_COUNT],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PrivateTxError {
    InvalidDepth,
    InvalidBranchLength,
    NegativeAmount,
    BalanceMismatch,
    MerkleRootMismatch,
    TruncatedInput,
    TrailingInput,
    Overflow,
}

impl fmt::Display for PrivateTxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            PrivateTxError::InvalidDepth => "invalid Merkle depth",
            PrivateTxError::InvalidBranchLength => "invalid Merkle branch length",
            PrivateTxError::NegativeAmount => "negative note amount",
            PrivateTxError::BalanceMismatch => "input and output balances differ",
            PrivateTxError::MerkleRootMismatch => "Merkle branch root mismatch",
            PrivateTxError::TruncatedInput => "private transaction input is truncated",
            PrivateTxError::TrailingInput => "private transaction input has trailing bytes",
            PrivateTxError::Overflow => "private transaction arithmetic overflowed",
        };
        f.write_str(msg)
    }
}

impl Error for PrivateTxError {}

pub fn generate_private_tx_case(depth: usize) -> PrivateTxInput {
    assert!(
        depth <= u64::BITS as usize,
        "private_tx depth exceeds u64 path bits"
    );

    let mut rng = StdRng::seed_from_u64(0x7072_6976_7478 ^ depth as u64);
    let input_notes = [
        PrivateTxNote {
            owner: rng.gen_range(1..10_000),
            amount: 70,
        },
        PrivateTxNote {
            owner: rng.gen_range(10_000..20_000),
            amount: 50,
        },
    ];
    let output_notes = [
        PrivateTxNote {
            owner: rng.gen_range(20_000..30_000),
            amount: 80,
        },
        PrivateTxNote {
            owner: rng.gen_range(30_000..40_000),
            amount: 40,
        },
    ];

    let input_branches = input_notes
        .clone()
        .map(|note| generate_branch(&mut rng, depth, note_commitment(&note).unwrap()));

    PrivateTxInput {
        depth,
        input_notes,
        output_notes,
        input_branches,
    }
}

pub fn generate_private_tx_input(depth: usize) -> (Vec<u8>, Vec<u8>) {
    let input = generate_private_tx_case(depth);
    let expected_output = evaluate_private_tx(&input)
        .expect("generated private_tx input should evaluate")
        .to_vec();
    let encoded = encode_private_tx_input(&input);
    (encoded, expected_output)
}

pub fn encode_private_tx_input(input: &PrivateTxInput) -> Vec<u8> {
    let mut out = Vec::with_capacity(4 + 4 * 16 + 2 * (16 + input.depth * 8));
    write_u32(&mut out, input.depth as u32);
    for note in &input.input_notes {
        encode_note(&mut out, note);
    }
    for note in &input.output_notes {
        encode_note(&mut out, note);
    }
    for branch in &input.input_branches {
        write_i64(&mut out, branch.expected_root);
        write_u64(&mut out, branch.path_index);
        for sibling in &branch.siblings {
            write_i64(&mut out, *sibling);
        }
    }
    out
}

pub fn decode_private_tx_input(bytes: &[u8]) -> Result<PrivateTxInput, PrivateTxError> {
    let mut cursor = Cursor::new(bytes);
    let depth = cursor.read_u32()? as usize;
    if depth > u64::BITS as usize {
        return Err(PrivateTxError::InvalidDepth);
    }

    let input_notes = [cursor.read_note()?, cursor.read_note()?];
    let output_notes = [cursor.read_note()?, cursor.read_note()?];
    let input_branches = [cursor.read_branch(depth)?, cursor.read_branch(depth)?];

    if !cursor.is_finished() {
        return Err(PrivateTxError::TrailingInput);
    }

    Ok(PrivateTxInput {
        depth,
        input_notes,
        output_notes,
        input_branches,
    })
}

pub fn evaluate_private_tx_bytes(
    bytes: &[u8],
) -> Result<[u8; PRIVATE_TX_PUBLIC_OUTPUT_BYTES], PrivateTxError> {
    let input = decode_private_tx_input(bytes)?;
    evaluate_private_tx(&input)
}

pub fn evaluate_private_tx(
    input: &PrivateTxInput,
) -> Result<[u8; PRIVATE_TX_PUBLIC_OUTPUT_BYTES], PrivateTxError> {
    if input.depth > u64::BITS as usize {
        return Err(PrivateTxError::InvalidDepth);
    }

    for branch in &input.input_branches {
        if branch.siblings.len() != input.depth {
            return Err(PrivateTxError::InvalidBranchLength);
        }
    }

    let mut input_sum = 0i64;
    for note in &input.input_notes {
        ensure_nonnegative(note.amount)?;
        input_sum = checked_hash(input_sum, note.amount)?;
    }

    let mut output_sum = 0i64;
    for note in &input.output_notes {
        ensure_nonnegative(note.amount)?;
        output_sum = checked_hash(output_sum, note.amount)?;
    }

    if input_sum != output_sum {
        return Err(PrivateTxError::BalanceMismatch);
    }

    let mut roots = [0i64; PRIVATE_TX_INPUT_COUNT];
    for ((note, branch), root) in input
        .input_notes
        .iter()
        .zip(input.input_branches.iter())
        .zip(roots.iter_mut())
    {
        let computed = evaluate_branch(note_commitment(note)?, branch)?;
        if computed != branch.expected_root {
            return Err(PrivateTxError::MerkleRootMismatch);
        }
        *root = computed;
    }

    let output_commitments = [
        note_commitment(&input.output_notes[0])?,
        note_commitment(&input.output_notes[1])?,
    ];

    let mut public_output = [0u8; PRIVATE_TX_PUBLIC_OUTPUT_BYTES];
    write_i64_at(&mut public_output, 0, roots[0]);
    write_i64_at(&mut public_output, 8, roots[1]);
    write_i64_at(&mut public_output, 16, output_commitments[0]);
    write_i64_at(&mut public_output, 24, output_commitments[1]);
    Ok(public_output)
}

pub fn note_commitment(note: &PrivateTxNote) -> Result<i64, PrivateTxError> {
    let owner = i64::try_from(note.owner).map_err(|_| PrivateTxError::Overflow)?;
    checked_hash(owner, note.amount)
}

fn generate_branch(rng: &mut StdRng, depth: usize, leaf: i64) -> PrivateTxMerkleBranch {
    let path_index = rng.r#gen::<u64>();
    let siblings: Vec<i64> = (0..depth).map(|_| rng.gen_range(1..10_000)).collect();
    let expected_root = evaluate_branch_from_parts(leaf, path_index, &siblings)
        .expect("generated branch should not overflow");

    PrivateTxMerkleBranch {
        expected_root,
        path_index,
        siblings,
    }
}

fn evaluate_branch(leaf: i64, branch: &PrivateTxMerkleBranch) -> Result<i64, PrivateTxError> {
    evaluate_branch_from_parts(leaf, branch.path_index, &branch.siblings)
}

fn evaluate_branch_from_parts(
    leaf: i64,
    path_index: u64,
    siblings: &[i64],
) -> Result<i64, PrivateTxError> {
    let mut acc = leaf;
    for (level, sibling) in siblings.iter().enumerate() {
        acc = if ((path_index >> level) & 1) == 1 {
            checked_hash(*sibling, acc)?
        } else {
            checked_hash(acc, *sibling)?
        };
    }
    Ok(acc)
}

fn ensure_nonnegative(value: i64) -> Result<(), PrivateTxError> {
    if value < 0 {
        Err(PrivateTxError::NegativeAmount)
    } else {
        Ok(())
    }
}

fn checked_hash(lhs: i64, rhs: i64) -> Result<i64, PrivateTxError> {
    lhs.checked_add(rhs).ok_or(PrivateTxError::Overflow)
}

fn encode_note(out: &mut Vec<u8>, note: &PrivateTxNote) {
    write_u64(out, note.owner);
    write_i64(out, note.amount);
}

fn write_u32(out: &mut Vec<u8>, value: u32) {
    out.extend_from_slice(&value.to_le_bytes());
}

fn write_u64(out: &mut Vec<u8>, value: u64) {
    out.extend_from_slice(&value.to_le_bytes());
}

fn write_i64(out: &mut Vec<u8>, value: i64) {
    out.extend_from_slice(&value.to_le_bytes());
}

fn write_i64_at(out: &mut [u8; PRIVATE_TX_PUBLIC_OUTPUT_BYTES], offset: usize, value: i64) {
    out[offset..offset + 8].copy_from_slice(&value.to_le_bytes());
}

struct Cursor<'a> {
    bytes: &'a [u8],
    offset: usize,
}

impl<'a> Cursor<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, offset: 0 }
    }

    fn read_u32(&mut self) -> Result<u32, PrivateTxError> {
        let mut bytes = [0u8; 4];
        bytes.copy_from_slice(self.read_exact(4)?);
        Ok(u32::from_le_bytes(bytes))
    }

    fn read_u64(&mut self) -> Result<u64, PrivateTxError> {
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(self.read_exact(8)?);
        Ok(u64::from_le_bytes(bytes))
    }

    fn read_i64(&mut self) -> Result<i64, PrivateTxError> {
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(self.read_exact(8)?);
        Ok(i64::from_le_bytes(bytes))
    }

    fn read_note(&mut self) -> Result<PrivateTxNote, PrivateTxError> {
        Ok(PrivateTxNote {
            owner: self.read_u64()?,
            amount: self.read_i64()?,
        })
    }

    fn read_branch(&mut self, depth: usize) -> Result<PrivateTxMerkleBranch, PrivateTxError> {
        let expected_root = self.read_i64()?;
        let path_index = self.read_u64()?;
        let mut siblings = Vec::with_capacity(depth);
        for _ in 0..depth {
            siblings.push(self.read_i64()?);
        }
        Ok(PrivateTxMerkleBranch {
            expected_root,
            path_index,
            siblings,
        })
    }

    fn read_exact(&mut self, len: usize) -> Result<&'a [u8], PrivateTxError> {
        let end = self
            .offset
            .checked_add(len)
            .ok_or(PrivateTxError::TruncatedInput)?;
        let bytes = self
            .bytes
            .get(self.offset..end)
            .ok_or(PrivateTxError::TruncatedInput)?;
        self.offset = end;
        Ok(bytes)
    }

    fn is_finished(&self) -> bool {
        self.offset == self.bytes.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generated_input_is_deterministic() {
        let first = generate_private_tx_case(32);
        let second = generate_private_tx_case(32);

        assert_eq!(first, second);
    }

    #[test]
    fn generated_input_roundtrips_and_evaluates() {
        let input = generate_private_tx_case(32);
        let encoded = encode_private_tx_input(&input);
        let decoded = decode_private_tx_input(&encoded).unwrap();

        assert_eq!(input, decoded);
        assert_eq!(
            evaluate_private_tx(&input).unwrap(),
            evaluate_private_tx_bytes(&encoded).unwrap()
        );
    }

    #[test]
    fn corrupted_branch_fails() {
        let mut input = generate_private_tx_case(32);
        input.input_branches[0].siblings[0] += 1;

        assert_eq!(
            evaluate_private_tx(&input).unwrap_err(),
            PrivateTxError::MerkleRootMismatch
        );
    }

    #[test]
    fn negative_balance_fails() {
        let mut input = generate_private_tx_case(32);
        input.output_notes[0].amount = -1;

        assert_eq!(
            evaluate_private_tx(&input).unwrap_err(),
            PrivateTxError::NegativeAmount
        );
    }

    #[test]
    fn input_output_sum_must_match() {
        let mut input = generate_private_tx_case(32);
        input.output_notes[0].amount += 1;

        assert_eq!(
            evaluate_private_tx(&input).unwrap_err(),
            PrivateTxError::BalanceMismatch
        );
    }
}
