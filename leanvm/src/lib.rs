use std::collections::BTreeMap;
use std::mem::size_of;
use std::sync::Once;

use lean_backend::{ArenaVec, PrimeCharacteristicRing, PrimeField32, poseidon16_compress};
use lean_compiler::{ProgramSource, compile_program};
use lean_prover::{
    default_whir_config,
    prove_execution::{ExecutionProof, prove_execution},
    verify_execution::verify_execution,
};
use lean_vm::{Bytecode, ExecutionWitness, F, Hints, PUBLIC_INPUT_LEN, try_execute_bytecode};
use utils::harness::{AuditStatus, BenchProperties};
use utils::private_tx::{
    PRIVATE_TX_INPUT_COUNT, PrivateTxInput, evaluate_private_tx, generate_private_tx_case,
    note_commitment,
};
use utils::targeted::MERKLE_DEPTH;

pub const PRIVATE_TX_DEPTH: usize = 32;
const PRIVATE_TX_FIELD_OUTPUTS: usize = 4;
const PRIVATE_TX_BRANCH_CELLS: usize = 1 + PRIVATE_TX_DEPTH + PRIVATE_TX_DEPTH;
const PRIVATE_TX_WITNESS_CELLS: usize = 8 + PRIVATE_TX_INPUT_COUNT * PRIVATE_TX_BRANCH_CELLS;
const PRIVATE_TX_PROGRAM: &str = include_str!("../guest/private_tx/main.py");
const CONSTANT_OVERHEAD_PROGRAM: &str = include_str!("../guest/constant_overhead/main.py");
const MERKLE_FAKE_PROGRAM: &str = include_str!("../guest/merkle_fake/main.py");
const HASH_POSEIDON16_PROGRAM: &str = include_str!("../guest/hash_poseidon16/main.py");
const MERKLE_POSEIDON16_PROGRAM: &str = include_str!("../guest/merkle_poseidon16/main.py");
const POSEIDON16_DIGEST_LEN: usize = 8;
const POSEIDON16_INPUT_LEN: usize = 16;
const LEAN_HASH_COUNTS: [usize; 2] = [128, 2048];
const LEAN_BRANCH_COUNTS: [usize; 2] = [4, 32];

static PROVER_SETUP: Once = Once::new();

pub type PrivateTxProof = ExecutionProof;
pub type LeanProof = ExecutionProof;

pub struct PreparedLean<'a> {
    bytecode: &'a Bytecode,
    public_input: [F; PUBLIC_INPUT_LEN],
    witness: ExecutionWitness,
    expected_output: Vec<u8>,
    output_field_count: usize,
}

pub type PreparedPrivateTx<'a> = PreparedLean<'a>;

impl<'a> PreparedLean<'a> {
    pub fn bytecode(&self) -> &Bytecode {
        self.bytecode
    }

    pub fn public_input(&self) -> &[F; PUBLIC_INPUT_LEN] {
        &self.public_input
    }

    pub fn witness(&self) -> &ExecutionWitness {
        &self.witness
    }

    pub fn expected_output(&self) -> &[u8] {
        &self.expected_output
    }

    pub fn output_field_count(&self) -> usize {
        self.output_field_count
    }
}

pub fn leanvm_bench_properties() -> BenchProperties {
    BenchProperties::new(
        "leanVM",
        "KoalaBear",
        "SuperSpartan + LogUp",
        Some("WHIR"),
        "AIR",
        false,
        true,
        124,
        true,
        true,
        AuditStatus::NotAudited,
        Some("leanVM"),
    )
}

pub fn compile_private_tx() -> Bytecode {
    compile_program(&ProgramSource::Raw(PRIVATE_TX_PROGRAM.to_string()))
}

pub fn compile_constant_overhead() -> Bytecode {
    compile_program(&ProgramSource::Raw(CONSTANT_OVERHEAD_PROGRAM.to_string()))
}

pub fn compile_merkle_fake() -> BTreeMap<usize, Bytecode> {
    compile_sized_programs(
        MERKLE_FAKE_PROGRAM,
        "BRANCH_COUNT_PLACEHOLDER",
        &LEAN_BRANCH_COUNTS,
    )
}

pub fn compile_hash_poseidon16() -> BTreeMap<usize, Bytecode> {
    compile_sized_programs(
        HASH_POSEIDON16_PROGRAM,
        "HASH_COUNT_PLACEHOLDER",
        &LEAN_HASH_COUNTS,
    )
}

pub fn compile_merkle_poseidon16() -> BTreeMap<usize, Bytecode> {
    compile_sized_programs(
        MERKLE_POSEIDON16_PROGRAM,
        "BRANCH_COUNT_PLACEHOLDER",
        &LEAN_BRANCH_COUNTS,
    )
}

fn compile_sized_programs(
    template: &str,
    placeholder: &str,
    sizes: &[usize],
) -> BTreeMap<usize, Bytecode> {
    sizes
        .iter()
        .map(|size| {
            let source = template.replace(placeholder, &size.to_string());
            (*size, compile_program(&ProgramSource::Raw(source)))
        })
        .collect()
}

pub fn prepare_private_tx(depth: usize, bytecode: &Bytecode) -> PreparedPrivateTx<'_> {
    assert_eq!(
        depth, PRIVATE_TX_DEPTH,
        "leanVM private_tx only supports depth {PRIVATE_TX_DEPTH}"
    );
    let input = generate_private_tx_case(depth);
    prepare_private_tx_input(&input, bytecode).expect("generated private_tx input should prepare")
}

pub fn prepare_constant_overhead(_input_size: usize, bytecode: &Bytecode) -> PreparedLean<'_> {
    let mut public_input = [F::ZERO; PUBLIC_INPUT_LEN];
    public_input[0] = F::from_u64(4);
    let witness = build_single_label_witness(bytecode, "constant_overhead", &[F::from_u64(2)]);
    let expected_output = public_input_output_bytes(&public_input, 1);
    PreparedLean {
        bytecode,
        public_input,
        witness,
        expected_output,
        output_field_count: 1,
    }
}

pub fn prepare_merkle_fake(
    branch_count: usize,
    bytecodes: &BTreeMap<usize, Bytecode>,
) -> PreparedLean<'_> {
    let bytecode = sized_bytecode(bytecodes, branch_count, "merkle_fake");
    let (_, expected_output) = utils::targeted::generate_fake_merkle_input(branch_count);
    let mut public_input = [F::ZERO; PUBLIC_INPUT_LEN];
    for i in 0..4 {
        public_input[i] = fe_from_i64(read_i64_le(&expected_output[i * 8..(i + 1) * 8]));
    }
    let bits = fake_merkle_bits(branch_count);
    let witness = build_single_label_witness(bytecode, "merkle_fake_bits", &bits);
    PreparedLean {
        bytecode,
        public_input,
        witness,
        expected_output,
        output_field_count: 4,
    }
}

pub fn prepare_hash_poseidon16(
    hash_count: usize,
    bytecodes: &BTreeMap<usize, Bytecode>,
) -> PreparedLean<'_> {
    let bytecode = sized_bytecode(bytecodes, hash_count, "hash_poseidon16");
    let (witness_fields, output_fields) = poseidon16_hash_case(hash_count);
    prepare_poseidon16_fields(bytecode, "poseidon16_inputs", witness_fields, output_fields)
}

pub fn prepare_merkle_poseidon16(
    branch_count: usize,
    bytecodes: &BTreeMap<usize, Bytecode>,
) -> PreparedLean<'_> {
    let bytecode = sized_bytecode(bytecodes, branch_count, "merkle_poseidon16");
    let (witness_fields, output_fields) = poseidon16_merkle_case(branch_count);
    prepare_poseidon16_fields(bytecode, "merkle_poseidon16", witness_fields, output_fields)
}

fn prepare_poseidon16_fields<'a>(
    bytecode: &'a Bytecode,
    label: &'static str,
    witness_fields: Vec<F>,
    output_fields: [F; POSEIDON16_DIGEST_LEN],
) -> PreparedLean<'a> {
    let mut public_input = [F::ZERO; PUBLIC_INPUT_LEN];
    public_input[..POSEIDON16_DIGEST_LEN].copy_from_slice(&output_fields);
    let witness = build_single_label_witness(bytecode, label, &witness_fields);
    let expected_output = public_input_output_bytes(&public_input, POSEIDON16_DIGEST_LEN);
    PreparedLean {
        bytecode,
        public_input,
        witness,
        expected_output,
        output_field_count: POSEIDON16_DIGEST_LEN,
    }
}

pub fn prove_private_tx<SharedState>(
    prepared: &PreparedPrivateTx<'_>,
    _: &SharedState,
) -> PrivateTxProof {
    prove_lean_bench(prepared, &())
}

pub fn prove_lean_bench<SharedState>(prepared: &PreparedLean<'_>, _: &SharedState) -> LeanProof {
    setup_prover();
    prove_execution(
        prepared.bytecode,
        &prepared.public_input,
        &prepared.witness,
        &default_whir_config(1),
        false,
    )
    .expect("prove failed")
}

pub fn verify_private_tx<SharedState>(
    prepared: &PreparedPrivateTx<'_>,
    proof: &PrivateTxProof,
    _: &SharedState,
) {
    verify_lean_bench(prepared, proof, &())
}

pub fn verify_lean_bench<SharedState>(
    prepared: &PreparedLean<'_>,
    proof: &LeanProof,
    _: &SharedState,
) {
    verify_execution(
        prepared.bytecode,
        &prepared.public_input,
        proof.proof.clone(),
    )
    .expect("verify failed");
    let actual_output =
        public_input_output_bytes(&prepared.public_input, prepared.output_field_count);
    assert_eq!(
        actual_output.as_slice(),
        prepared.expected_output(),
        "leanVM public output mismatch"
    );
}

pub fn preprocessing_size<SharedState>(prepared: &PreparedLean<'_>, _: &SharedState) -> usize {
    prepared.bytecode.instructions_multilinear().len() * size_of::<u32>()
}

pub fn proof_size<SharedState>(proof: &LeanProof, _: &SharedState) -> usize {
    proof.proof.proof_size_fe() * size_of::<u32>()
}

pub fn execution_cycles(prepared: &PreparedLean<'_>) -> u64 {
    try_execute_bytecode(
        prepared.bytecode,
        &prepared.public_input,
        &prepared.witness,
        false,
    )
    .expect("execute failed")
    .n_cycles() as u64
}

pub fn num_constraints<SharedState>(_: &PreparedLean<'_>, _: &SharedState) -> usize {
    0
}

fn setup_prover() {
    PROVER_SETUP.call_once(|| {
        lean_backend::enable_arena();
        lean_backend::parallel::init();
    });
}

fn prepare_private_tx_input<'a>(
    input: &PrivateTxInput,
    bytecode: &'a Bytecode,
) -> anyhow::Result<PreparedPrivateTx<'a>> {
    let expected_output = evaluate_private_tx(input)?.to_vec();
    let prepared = prepare_private_tx_input_unchecked(input, bytecode)?;
    if prepared.expected_output != expected_output {
        anyhow::bail!("private_tx public input does not match expected output");
    }
    Ok(PreparedPrivateTx {
        expected_output,
        output_field_count: PRIVATE_TX_FIELD_OUTPUTS,
        ..prepared
    })
}

fn prepare_private_tx_input_unchecked<'a>(
    input: &PrivateTxInput,
    bytecode: &'a Bytecode,
) -> anyhow::Result<PreparedPrivateTx<'a>> {
    let public_input = public_input_from_claims(input)?;
    let witness = build_witness(input, bytecode)?;
    let expected_output = public_input_output_bytes(&public_input, PRIVATE_TX_FIELD_OUTPUTS);
    Ok(PreparedPrivateTx {
        bytecode,
        public_input,
        witness,
        expected_output,
        output_field_count: PRIVATE_TX_FIELD_OUTPUTS,
    })
}

fn build_witness(input: &PrivateTxInput, bytecode: &Bytecode) -> anyhow::Result<ExecutionWitness> {
    if input.depth != PRIVATE_TX_DEPTH {
        anyhow::bail!("leanVM private_tx only supports depth {PRIVATE_TX_DEPTH}");
    }
    for branch in &input.input_branches {
        if branch.siblings.len() != PRIVATE_TX_DEPTH {
            anyhow::bail!("invalid private_tx branch length");
        }
    }

    let mut fields = Vec::with_capacity(PRIVATE_TX_WITNESS_CELLS);
    for note in &input.input_notes {
        fields.push(fe_from_u64(note.owner));
        fields.push(fe_from_i64(note.amount));
    }
    for note in &input.output_notes {
        fields.push(fe_from_u64(note.owner));
        fields.push(fe_from_i64(note.amount));
    }
    for branch in &input.input_branches {
        fields.push(fe_from_i64(branch.expected_root));
        for level in 0..PRIVATE_TX_DEPTH {
            fields.push(fe_from_u64((branch.path_index >> level) & 1));
        }
        for sibling in &branch.siblings {
            fields.push(fe_from_i64(*sibling));
        }
    }

    let mut hints = Hints::default();
    let entries = ArenaVec::from_iter([ArenaVec::from_slice(&fields)]);
    hints.insert(bytecode, "private_tx", entries);
    Ok(ExecutionWitness {
        hints,
        ..Default::default()
    })
}

fn build_single_label_witness(
    bytecode: &Bytecode,
    label: &'static str,
    fields: &[F],
) -> ExecutionWitness {
    let mut hints = Hints::default();
    let entries = ArenaVec::from_iter([ArenaVec::from_slice(fields)]);
    hints.insert(bytecode, label, entries);
    ExecutionWitness {
        hints,
        ..Default::default()
    }
}

fn public_input_from_claims(input: &PrivateTxInput) -> anyhow::Result<[F; PUBLIC_INPUT_LEN]> {
    let mut public_input = [F::ZERO; PUBLIC_INPUT_LEN];
    public_input[0] = fe_from_i64(input.input_branches[0].expected_root);
    public_input[1] = fe_from_i64(input.input_branches[1].expected_root);
    public_input[2] = fe_from_i64(note_commitment(&input.output_notes[0])?);
    public_input[3] = fe_from_i64(note_commitment(&input.output_notes[1])?);
    Ok(public_input)
}

fn public_input_output_bytes(public_input: &[F; PUBLIC_INPUT_LEN], field_count: usize) -> Vec<u8> {
    let mut output = vec![0u8; field_count * 8];
    for i in 0..field_count {
        let value = i64::from(public_input[i].as_canonical_u32());
        output[i * 8..(i + 1) * 8].copy_from_slice(&value.to_le_bytes());
    }
    output
}

fn fe_from_u64(value: u64) -> F {
    F::from_u64(value)
}

fn fe_from_i64(value: i64) -> F {
    F::from_i64(value)
}

fn sized_bytecode<'a>(
    bytecodes: &'a BTreeMap<usize, Bytecode>,
    size: usize,
    bench_name: &str,
) -> &'a Bytecode {
    bytecodes
        .get(&size)
        .unwrap_or_else(|| panic!("leanVM {bench_name} does not support input_size={size}"))
}

fn fake_merkle_bits(branch_count: usize) -> Vec<F> {
    let mut bits = Vec::with_capacity(branch_count * MERKLE_DEPTH);
    for branch in 0..branch_count {
        let path = path_index(branch as u64);
        for level in 0..MERKLE_DEPTH {
            bits.push(F::from_u64((path >> level) & 1));
        }
    }
    bits
}

fn poseidon16_hash_case(hash_count: usize) -> (Vec<F>, [F; POSEIDON16_DIGEST_LEN]) {
    let mut witness = Vec::with_capacity(hash_count * POSEIDON16_INPUT_LEN);
    let mut folded = [F::ZERO; POSEIDON16_DIGEST_LEN];

    for index in 0..hash_count {
        let input = poseidon16_input(0x6861_7368, index as u64, 0);
        witness.extend_from_slice(&input);
        let output = poseidon16_compress(input);
        for cell in 0..POSEIDON16_DIGEST_LEN {
            folded[cell] = folded[cell] + output[cell];
        }
    }

    (witness, folded)
}

fn poseidon16_merkle_case(branch_count: usize) -> (Vec<F>, [F; POSEIDON16_DIGEST_LEN]) {
    let branch_cells = POSEIDON16_DIGEST_LEN + MERKLE_DEPTH + MERKLE_DEPTH * POSEIDON16_DIGEST_LEN;
    let mut witness = Vec::with_capacity(branch_count * branch_cells);
    let mut folded = [F::ZERO; POSEIDON16_DIGEST_LEN];

    for branch in 0..branch_count {
        let mut acc = poseidon16_digest(0x6c65_6166, branch as u64, 0);
        witness.extend_from_slice(&acc);

        let path = path_index(branch as u64);
        for level in 0..MERKLE_DEPTH {
            witness.push(F::from_u64((path >> level) & 1));
        }

        let mut siblings = [[F::ZERO; POSEIDON16_DIGEST_LEN]; MERKLE_DEPTH];
        for (level, sibling) in siblings.iter_mut().enumerate() {
            *sibling = poseidon16_digest(0x7369_626c, branch as u64, level as u64);
            witness.extend_from_slice(sibling);
        }

        for (level, sibling) in siblings.iter().enumerate() {
            let mut input = [F::ZERO; POSEIDON16_INPUT_LEN];
            if ((path >> level) & 1) == 1 {
                input[..POSEIDON16_DIGEST_LEN].copy_from_slice(sibling);
                input[POSEIDON16_DIGEST_LEN..].copy_from_slice(&acc);
            } else {
                input[..POSEIDON16_DIGEST_LEN].copy_from_slice(&acc);
                input[POSEIDON16_DIGEST_LEN..].copy_from_slice(sibling);
            }
            acc = poseidon16_compress(input);
        }

        for cell in 0..POSEIDON16_DIGEST_LEN {
            folded[cell] = folded[cell] + acc[cell];
        }
    }

    (witness, folded)
}

fn poseidon16_input(domain: u64, index: u64, offset: u64) -> [F; POSEIDON16_INPUT_LEN] {
    let mut input = [F::ZERO; POSEIDON16_INPUT_LEN];
    for (cell, value) in input.iter_mut().enumerate() {
        *value = deterministic_field(domain, index, offset + cell as u64);
    }
    input
}

fn poseidon16_digest(domain: u64, index: u64, offset: u64) -> [F; POSEIDON16_DIGEST_LEN] {
    let mut digest = [F::ZERO; POSEIDON16_DIGEST_LEN];
    for (cell, value) in digest.iter_mut().enumerate() {
        *value = deterministic_field(domain, index, offset + cell as u64);
    }
    digest
}

fn deterministic_field(domain: u64, index: u64, offset: u64) -> F {
    let mut state = domain
        ^ index.wrapping_mul(0x9e37_79b9_7f4a_7c15)
        ^ offset.wrapping_mul(0xbf58_476d_1ce4_e5b9);
    F::from_u64(splitmix64(&mut state) & 0xffff)
}

fn path_index(branch: u64) -> u64 {
    let mut state = 0x7061_7468_5f69_6478 ^ branch;
    splitmix64(&mut state)
}

fn splitmix64(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9e37_79b9_7f4a_7c15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^ (z >> 31)
}

fn read_i64_le(bytes: &[u8]) -> i64 {
    let mut value = [0u8; 8];
    value.copy_from_slice(bytes);
    i64::from_le_bytes(value)
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::private_tx::PrivateTxError;

    #[test]
    fn witness_public_input_matches_utils_output() {
        let bytecode = compile_private_tx();
        let input = generate_private_tx_case(PRIVATE_TX_DEPTH);
        let prepared = prepare_private_tx_input(&input, &bytecode).unwrap();
        let expected = evaluate_private_tx(&input).unwrap().to_vec();

        assert_eq!(prepared.expected_output(), expected.as_slice());
        assert_eq!(
            public_input_output_bytes(prepared.public_input(), prepared.output_field_count())
                .as_slice(),
            expected.as_slice()
        );
    }

    #[test]
    fn constant_overhead_executes() {
        let bytecode = compile_constant_overhead();
        let prepared = prepare_constant_overhead(1, &bytecode);
        let result = try_execute_bytecode(
            prepared.bytecode(),
            prepared.public_input(),
            prepared.witness(),
            false,
        )
        .unwrap();

        assert!(result.n_cycles() > 0);
        assert_eq!(prepared.expected_output(), 4u64.to_le_bytes());
    }

    #[test]
    fn poseidon16_hash_public_input_matches_reference() {
        let programs = compile_hash_poseidon16();
        let prepared = prepare_hash_poseidon16(128, &programs);
        let result = try_execute_bytecode(
            prepared.bytecode(),
            prepared.public_input(),
            prepared.witness(),
            false,
        )
        .unwrap();

        assert!(result.n_cycles() > 0);
        assert_eq!(
            public_input_output_bytes(prepared.public_input(), prepared.output_field_count()),
            prepared.expected_output()
        );
    }

    #[test]
    fn guest_executes_generated_input() {
        let bytecode = compile_private_tx();
        let prepared = prepare_private_tx(PRIVATE_TX_DEPTH, &bytecode);
        let result = try_execute_bytecode(
            prepared.bytecode(),
            prepared.public_input(),
            prepared.witness(),
            false,
        )
        .unwrap();

        assert!(result.n_cycles() > 0);
    }

    #[test]
    fn corrupted_branch_fails_execution() {
        let bytecode = compile_private_tx();
        let mut input = generate_private_tx_case(PRIVATE_TX_DEPTH);
        input.input_branches[0].expected_root += 1;
        let prepared = prepare_private_tx_input_unchecked(&input, &bytecode).unwrap();

        assert!(
            try_execute_bytecode(
                prepared.bytecode(),
                prepared.public_input(),
                prepared.witness(),
                false,
            )
            .is_err()
        );
    }

    #[test]
    fn balance_mismatch_fails_execution() {
        let bytecode = compile_private_tx();
        let mut input = generate_private_tx_case(PRIVATE_TX_DEPTH);
        input.output_notes[0].amount += 1;
        let prepared = prepare_private_tx_input_unchecked(&input, &bytecode).unwrap();

        assert_eq!(
            evaluate_private_tx(&input).unwrap_err(),
            PrivateTxError::BalanceMismatch
        );
        assert!(
            try_execute_bytecode(
                prepared.bytecode(),
                prepared.public_input(),
                prepared.witness(),
                false,
            )
            .is_err()
        );
    }

    #[test]
    fn unsupported_depth_is_rejected() {
        let bytecode = compile_private_tx();
        let input = generate_private_tx_case(16);

        assert!(prepare_private_tx_input(&input, &bytecode).is_err());
    }
}
