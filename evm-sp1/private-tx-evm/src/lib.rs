use revm::{
    Context, ExecuteEvm, MainBuilder, MainContext,
    context::{BlockEnv, CfgEnv, TxEnv},
    context_interface::result::{ExecutionResult, Output},
    database::{CacheDB, EmptyDB},
    primitives::{Address, Bytes, U256, hardfork::SpecId},
    state::{AccountInfo, Bytecode},
};

pub const PRIVATE_TX_DEPTH: usize = 32;
pub const PUBLIC_OUTPUT_BYTES: usize = 32;
pub const ABI_OUTPUT_BYTES: usize = 128;
pub const CONTRACT_ADDRESS: Address = Address::with_last_byte(0x42);
pub const CALLER_ADDRESS: Address = Address::with_last_byte(0x7a);
pub const FUNCTION_SELECTOR: [u8; 4] = [0xa3, 0xa8, 0xa2, 0x26];

const NOTE_COUNT: usize = 4;
const INPUT_COUNT: usize = 2;
const ABI_WORD_BYTES: usize = 32;
const DEPTH_BYTES: usize = 4;
const NOTE_BYTES: usize = 16;
const BRANCH_HEADER_BYTES: usize = 16;
const CANONICAL_INPUT_BYTES: usize = DEPTH_BYTES
    + NOTE_COUNT * NOTE_BYTES
    + INPUT_COUNT * (BRANCH_HEADER_BYTES + PRIVATE_TX_DEPTH * 8);
const ABI_WORD_COUNT: usize = 4 + 4 + 2 + 2 + 64;
const ABI_CALLDATA_BYTES: usize = 4 + ABI_WORD_COUNT * ABI_WORD_BYTES;

const MEM_AMOUNT0: u16 = 0x80;
const MEM_AMOUNT1: u16 = 0xa0;
const MEM_AMOUNT2: u16 = 0xc0;
const MEM_AMOUNT3: u16 = 0xe0;
const MEM_ROOT0: u16 = 0x100;
const MEM_ROOT1: u16 = 0x120;
const I64_NONNEGATIVE_LIMIT: u64 = 1u64 << 63;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PrivateTxEvmError {
    UnsupportedDepth,
    InvalidInputLength,
    InvalidOutputLength,
    InvalidSignExtension,
    EvmExecutionFailed(String),
    EvmReverted,
    EvmHalted(String),
}

pub fn encode_private_tx_calldata(input: &[u8]) -> Result<Vec<u8>, PrivateTxEvmError> {
    let tx = ParsedPrivateTx::decode(input)?;
    let mut out = Vec::with_capacity(ABI_CALLDATA_BYTES);
    out.extend_from_slice(&FUNCTION_SELECTOR);

    for owner in tx.owners {
        encode_u64_word(&mut out, owner);
    }
    for amount in tx.amounts {
        encode_i64_word(&mut out, amount);
    }
    for root in tx.expected_roots {
        encode_i64_word(&mut out, root);
    }
    for path_index in tx.path_indices {
        encode_u64_word(&mut out, path_index);
    }
    for sibling in tx.siblings {
        encode_i64_word(&mut out, sibling);
    }

    Ok(out)
}

pub fn execute_private_tx(input: &[u8]) -> Result<[u8; PUBLIC_OUTPUT_BYTES], PrivateTxEvmError> {
    let calldata = encode_private_tx_calldata(input)?;
    let abi_output = execute_private_tx_calldata(&calldata)?;
    decode_abi_output(&abi_output)
}

pub fn execute_private_tx_calldata(calldata: &[u8]) -> Result<Vec<u8>, PrivateTxEvmError> {
    execute_private_tx_calldata_with_code(calldata, &contract_bytecode())
}

pub fn execute_private_tx_calldata_with_code(
    calldata: &[u8],
    code: &[u8],
) -> Result<Vec<u8>, PrivateTxEvmError> {
    let mut db = CacheDB::new(EmptyDB::default());
    let bytecode = Bytecode::new_legacy(Bytes::copy_from_slice(code));
    db.insert_account_info(CONTRACT_ADDRESS, AccountInfo::default().with_code(bytecode));

    let mut cfg: CfgEnv<SpecId> = CfgEnv::default();
    cfg.disable_nonce_check = true;
    cfg.disable_balance_check = true;
    cfg.disable_fee_charge = true;
    cfg.tx_chain_id_check = false;

    let mut evm = Context::mainnet()
        .with_db(db)
        .with_cfg(cfg)
        .with_block(BlockEnv::default())
        .build_mainnet();

    let tx = TxEnv::builder()
        .caller(CALLER_ADDRESS)
        .gas_limit(10_000_000)
        .gas_price(0)
        .value(U256::ZERO)
        .call(CONTRACT_ADDRESS)
        .data(Bytes::copy_from_slice(calldata))
        .build()
        .map_err(|err| PrivateTxEvmError::EvmExecutionFailed(err.to_string()))?;

    match evm
        .transact(tx)
        .map_err(|err| PrivateTxEvmError::EvmExecutionFailed(err.to_string()))?
        .result
    {
        ExecutionResult::Success {
            output: Output::Call(output),
            ..
        } => Ok(output.to_vec()),
        ExecutionResult::Success { .. } => Err(PrivateTxEvmError::EvmExecutionFailed(
            "unexpected create output for call transaction".to_string(),
        )),
        ExecutionResult::Revert { .. } => Err(PrivateTxEvmError::EvmReverted),
        ExecutionResult::Halt { reason, .. } => {
            Err(PrivateTxEvmError::EvmHalted(format!("{reason:?}")))
        }
    }
}

pub fn decode_abi_output(output: &[u8]) -> Result<[u8; PUBLIC_OUTPUT_BYTES], PrivateTxEvmError> {
    if output.len() != ABI_OUTPUT_BYTES {
        return Err(PrivateTxEvmError::InvalidOutputLength);
    }

    let mut canonical = [0u8; PUBLIC_OUTPUT_BYTES];
    for i in 0..4 {
        let word = &output[i * ABI_WORD_BYTES..(i + 1) * ABI_WORD_BYTES];
        let value = decode_i64_word(word)?;
        canonical[i * 8..(i + 1) * 8].copy_from_slice(&value.to_le_bytes());
    }
    Ok(canonical)
}

pub fn contract_bytecode() -> Vec<u8> {
    let mut asm = EvmAssembler::default();

    asm.push_u16(ABI_CALLDATA_BYTES as u16);
    asm.op(0x36);
    asm.op(0x14);
    asm.require_true();

    for i in 0..NOTE_COUNT {
        asm.load_calldata_word(calldata_amount_offset(i));
        asm.op(0x80);
        asm.push_u16(amount_mem_offset(i));
        asm.op(0x52);
        asm.push_u64(I64_NONNEGATIVE_LIMIT);
        asm.op(0x90);
        asm.op(0x10);
        asm.require_true();
    }

    asm.mload(MEM_AMOUNT0);
    asm.mload(MEM_AMOUNT1);
    asm.op(0x01);
    asm.mload(MEM_AMOUNT2);
    asm.mload(MEM_AMOUNT3);
    asm.op(0x01);
    asm.op(0x14);
    asm.require_true();

    asm.note_commitment(0);
    for i in 0..PRIVATE_TX_DEPTH {
        asm.branch_hash(0, i);
    }
    asm.op(0x80);
    asm.push_u16(MEM_ROOT0);
    asm.op(0x52);
    asm.load_calldata_word(calldata_expected_root_offset(0));
    asm.op(0x14);
    asm.require_true();

    asm.note_commitment(1);
    for i in 0..PRIVATE_TX_DEPTH {
        asm.branch_hash(1, i);
    }
    asm.op(0x80);
    asm.push_u16(MEM_ROOT1);
    asm.op(0x52);
    asm.load_calldata_word(calldata_expected_root_offset(1));
    asm.op(0x14);
    asm.require_true();

    asm.note_commitment(2);
    asm.push_u16(0x40);
    asm.op(0x52);
    asm.note_commitment(3);
    asm.push_u16(0x60);
    asm.op(0x52);
    asm.mload(MEM_ROOT0);
    asm.push_u8(0x00);
    asm.op(0x52);
    asm.mload(MEM_ROOT1);
    asm.push_u8(0x20);
    asm.op(0x52);

    asm.push_u8(0x80);
    asm.push_u8(0x00);
    asm.op(0xf3);
    asm.finish()
}

fn calldata_owner_offset(index: usize) -> u16 {
    calldata_word_offset(index)
}

fn calldata_amount_offset(index: usize) -> u16 {
    calldata_word_offset(4 + index)
}

fn calldata_expected_root_offset(index: usize) -> u16 {
    calldata_word_offset(8 + index)
}

fn calldata_path_index_offset(index: usize) -> u16 {
    calldata_word_offset(10 + index)
}

fn calldata_sibling_offset(branch: usize, index: usize) -> u16 {
    calldata_word_offset(12 + branch * PRIVATE_TX_DEPTH + index)
}

fn calldata_word_offset(index: usize) -> u16 {
    4 + (index * ABI_WORD_BYTES) as u16
}

fn amount_mem_offset(index: usize) -> u16 {
    match index {
        0 => MEM_AMOUNT0,
        1 => MEM_AMOUNT1,
        2 => MEM_AMOUNT2,
        3 => MEM_AMOUNT3,
        _ => unreachable!("private_tx has exactly four note amounts"),
    }
}

fn encode_u64_word(out: &mut Vec<u8>, value: u64) {
    out.extend_from_slice(&[0u8; 24]);
    out.extend_from_slice(&value.to_be_bytes());
}

fn encode_i64_word(out: &mut Vec<u8>, value: i64) {
    out.extend_from_slice(if value < 0 { &[0xff; 24] } else { &[0u8; 24] });
    out.extend_from_slice(&value.to_be_bytes());
}

fn decode_i64_word(word: &[u8]) -> Result<i64, PrivateTxEvmError> {
    let mut bytes = [0u8; 8];
    bytes.copy_from_slice(&word[24..32]);
    let value = i64::from_be_bytes(bytes);
    let expected_extension = if value < 0 { 0xff } else { 0x00 };
    if word[..24].iter().any(|byte| *byte != expected_extension) {
        return Err(PrivateTxEvmError::InvalidSignExtension);
    }
    Ok(value)
}

#[derive(Clone, Debug)]
struct ParsedPrivateTx {
    owners: [u64; NOTE_COUNT],
    amounts: [i64; NOTE_COUNT],
    expected_roots: [i64; INPUT_COUNT],
    path_indices: [u64; INPUT_COUNT],
    siblings: [i64; INPUT_COUNT * PRIVATE_TX_DEPTH],
}

impl ParsedPrivateTx {
    fn decode(input: &[u8]) -> Result<Self, PrivateTxEvmError> {
        if input.len() != CANONICAL_INPUT_BYTES {
            return Err(PrivateTxEvmError::InvalidInputLength);
        }

        let mut cursor = Cursor::new(input);
        let depth = cursor.read_u32_le()? as usize;
        if depth != PRIVATE_TX_DEPTH {
            return Err(PrivateTxEvmError::UnsupportedDepth);
        }

        let mut owners = [0u64; NOTE_COUNT];
        let mut amounts = [0i64; NOTE_COUNT];
        for i in 0..NOTE_COUNT {
            owners[i] = cursor.read_u64_le()?;
            amounts[i] = cursor.read_i64_le()?;
        }

        let mut expected_roots = [0i64; INPUT_COUNT];
        let mut path_indices = [0u64; INPUT_COUNT];
        let mut siblings = [0i64; INPUT_COUNT * PRIVATE_TX_DEPTH];
        for branch in 0..INPUT_COUNT {
            expected_roots[branch] = cursor.read_i64_le()?;
            path_indices[branch] = cursor.read_u64_le()?;
            for level in 0..PRIVATE_TX_DEPTH {
                siblings[branch * PRIVATE_TX_DEPTH + level] = cursor.read_i64_le()?;
            }
        }

        Ok(Self {
            owners,
            amounts,
            expected_roots,
            path_indices,
            siblings,
        })
    }
}

struct Cursor<'a> {
    input: &'a [u8],
    offset: usize,
}

impl<'a> Cursor<'a> {
    fn new(input: &'a [u8]) -> Self {
        Self { input, offset: 0 }
    }

    fn read_u32_le(&mut self) -> Result<u32, PrivateTxEvmError> {
        let mut bytes = [0u8; 4];
        bytes.copy_from_slice(self.read_exact(4)?);
        Ok(u32::from_le_bytes(bytes))
    }

    fn read_u64_le(&mut self) -> Result<u64, PrivateTxEvmError> {
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(self.read_exact(8)?);
        Ok(u64::from_le_bytes(bytes))
    }

    fn read_i64_le(&mut self) -> Result<i64, PrivateTxEvmError> {
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(self.read_exact(8)?);
        Ok(i64::from_le_bytes(bytes))
    }

    fn read_exact(&mut self, len: usize) -> Result<&'a [u8], PrivateTxEvmError> {
        let end = self
            .offset
            .checked_add(len)
            .ok_or(PrivateTxEvmError::InvalidInputLength)?;
        let bytes = self
            .input
            .get(self.offset..end)
            .ok_or(PrivateTxEvmError::InvalidInputLength)?;
        self.offset = end;
        Ok(bytes)
    }
}

#[derive(Default)]
struct EvmAssembler {
    code: Vec<u8>,
    revert_patches: Vec<usize>,
}

impl EvmAssembler {
    fn finish(mut self) -> Vec<u8> {
        let revert_label = self.code.len() as u16;
        for patch in &self.revert_patches {
            let [hi, lo] = revert_label.to_be_bytes();
            self.code[*patch] = hi;
            self.code[*patch + 1] = lo;
        }

        self.op(0x5b);
        self.push_u8(0x00);
        self.push_u8(0x00);
        self.op(0xfd);
        self.code
    }

    fn note_commitment(&mut self, note_index: usize) {
        self.load_calldata_word(calldata_owner_offset(note_index));
        self.mload(amount_mem_offset(note_index));
        self.op(0x01);
    }

    fn branch_hash(&mut self, branch: usize, level: usize) {
        self.load_calldata_word(calldata_path_index_offset(branch));
        self.push_u64(1u64 << level);
        self.op(0x16);
        self.op(0x15);
        self.op(0x61);
        let zero_bit_patch = self.code.len();
        self.code.extend_from_slice(&[0x00, 0x00]);
        self.op(0x57);

        self.load_calldata_word(calldata_sibling_offset(branch, level));
        self.op(0x01);
        self.op(0x61);
        let end_patch = self.code.len();
        self.code.extend_from_slice(&[0x00, 0x00]);
        self.op(0x56);

        self.patch_u16(zero_bit_patch, self.code.len() as u16);
        self.op(0x5b);
        self.load_calldata_word(calldata_sibling_offset(branch, level));
        self.op(0x90);
        self.op(0x01);

        self.patch_u16(end_patch, self.code.len() as u16);
        self.op(0x5b);
    }

    fn require_true(&mut self) {
        self.op(0x15);
        self.op(0x61);
        let patch = self.code.len();
        self.code.extend_from_slice(&[0x00, 0x00]);
        self.revert_patches.push(patch);
        self.op(0x57);
    }

    fn load_calldata_word(&mut self, offset: u16) {
        self.push_u16(offset);
        self.op(0x35);
    }

    fn mload(&mut self, offset: u16) {
        self.push_u16(offset);
        self.op(0x51);
    }

    fn push_u8(&mut self, value: u8) {
        self.op(0x60);
        self.op(value);
    }

    fn push_u16(&mut self, value: u16) {
        self.op(0x61);
        self.code.extend_from_slice(&value.to_be_bytes());
    }

    fn push_u64(&mut self, value: u64) {
        self.op(0x67);
        self.code.extend_from_slice(&value.to_be_bytes());
    }

    fn op(&mut self, opcode: u8) {
        self.code.push(opcode);
    }

    fn patch_u16(&mut self, patch: usize, value: u16) {
        let [hi, lo] = value.to_be_bytes();
        self.code[patch] = hi;
        self.code[patch + 1] = lo;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{path::Path, process::Command};

    fn valid_input() -> Vec<u8> {
        let mut input = Vec::new();
        input.extend_from_slice(&(PRIVATE_TX_DEPTH as u32).to_le_bytes());
        for (owner, amount) in [(1u64, 70i64), (2, 50), (3, 80), (4, 40)] {
            input.extend_from_slice(&owner.to_le_bytes());
            input.extend_from_slice(&amount.to_le_bytes());
        }
        for expected_root in [1 + 70 + 32, 2 + 50 + 64] {
            input.extend_from_slice(&(expected_root as i64).to_le_bytes());
            input.extend_from_slice(&0u64.to_le_bytes());
            for sibling in [if expected_root == 103 { 1i64 } else { 2i64 }; PRIVATE_TX_DEPTH] {
                input.extend_from_slice(&sibling.to_le_bytes());
            }
        }
        input
    }

    #[test]
    fn valid_execution_returns_canonical_output() {
        let output = execute_private_tx(&valid_input()).unwrap();
        assert_eq!(&output[0..8], &103i64.to_le_bytes());
        assert_eq!(&output[8..16], &116i64.to_le_bytes());
        assert_eq!(&output[16..24], &83i64.to_le_bytes());
        assert_eq!(&output[24..32], &44i64.to_le_bytes());
    }

    #[test]
    fn solidity_contract_behavior_matches_benchmark_bytecode() {
        let solidity_bytecode = compile_solidity_runtime_bytecode();

        assert_same_execution(&valid_input(), &solidity_bytecode);
        assert_same_execution(
            &input_with_nonzero_path_direction_bits(),
            &solidity_bytecode,
        );

        let mut balance_mismatch = valid_input();
        let output_amount_0 = 4 + 2 * 16 + 8;
        balance_mismatch[output_amount_0..output_amount_0 + 8]
            .copy_from_slice(&81i64.to_le_bytes());
        assert_same_execution(&balance_mismatch, &solidity_bytecode);

        let mut corrupted_root = valid_input();
        let first_root = 4 + 4 * 16;
        corrupted_root[first_root..first_root + 8].copy_from_slice(&999i64.to_le_bytes());
        assert_same_execution(&corrupted_root, &solidity_bytecode);
    }

    #[test]
    fn valid_execution_accepts_nonzero_path_direction_bits() {
        let input = input_with_nonzero_path_direction_bits();
        let output = execute_private_tx(&input).unwrap();
        assert_eq!(&output[0..8], &103i64.to_le_bytes());
        assert_eq!(&output[8..16], &116i64.to_le_bytes());
    }

    #[test]
    fn contract_bytecode_branches_on_each_path_bit() {
        let required_checks = 1 + NOTE_COUNT + 1 + INPUT_COUNT;
        let path_branches = INPUT_COUNT * PRIVATE_TX_DEPTH;
        assert_eq!(
            count_opcode(&contract_bytecode(), 0x57),
            required_checks + path_branches
        );
    }

    #[test]
    fn balance_mismatch_reverts() {
        let mut input = valid_input();
        let output_amount_0 = 4 + 2 * 16 + 8;
        input[output_amount_0..output_amount_0 + 8].copy_from_slice(&81i64.to_le_bytes());
        assert_eq!(
            execute_private_tx(&input).unwrap_err(),
            PrivateTxEvmError::EvmReverted
        );
    }

    #[test]
    fn corrupted_root_reverts() {
        let mut input = valid_input();
        let first_root = 4 + 4 * 16;
        input[first_root..first_root + 8].copy_from_slice(&999i64.to_le_bytes());
        assert_eq!(
            execute_private_tx(&input).unwrap_err(),
            PrivateTxEvmError::EvmReverted
        );
    }

    #[test]
    fn unsupported_depth_is_rejected_before_evm() {
        let mut input = valid_input();
        input[0..4].copy_from_slice(&16u32.to_le_bytes());
        assert_eq!(
            execute_private_tx(&input).unwrap_err(),
            PrivateTxEvmError::UnsupportedDepth
        );
    }

    fn input_with_nonzero_path_direction_bits() -> Vec<u8> {
        let mut input = valid_input();
        let first_path_index = branch_path_index_offset(0);
        let second_path_index = branch_path_index_offset(1);
        input[first_path_index..first_path_index + 8]
            .copy_from_slice(&0xaaaa_aaaau64.to_le_bytes());
        input[second_path_index..second_path_index + 8]
            .copy_from_slice(&0x5555_5555u64.to_le_bytes());
        input
    }

    fn assert_same_execution(input: &[u8], solidity_bytecode: &[u8]) {
        let calldata = encode_private_tx_calldata(input).unwrap();
        let benchmark_result = execute_private_tx_calldata(&calldata);
        let solidity_result = execute_private_tx_calldata_with_code(&calldata, solidity_bytecode);
        assert_eq!(solidity_result, benchmark_result);
    }

    fn compile_solidity_runtime_bytecode() -> Vec<u8> {
        let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
        let evm_sp1_dir = manifest_dir
            .parent()
            .expect("private-tx-evm crate should live under evm-sp1");
        let temp_dir = std::env::temp_dir().join(format!(
            "csp-benchmarks-private-tx-solidity-{}",
            std::process::id()
        ));
        let out_dir = temp_dir.join("out");
        let cache_dir = temp_dir.join("cache");

        let output = Command::new("forge")
            .arg("inspect")
            .arg("PrivateTx")
            .arg("deployedBytecode")
            .arg("--root")
            .arg(evm_sp1_dir)
            .arg("--contracts")
            .arg("contracts")
            .arg("--use")
            .arg("0.8.24")
            .arg("--no-metadata")
            .arg("--out")
            .arg(&out_dir)
            .arg("--cache-path")
            .arg(&cache_dir)
            .output()
            .expect("forge is required for Solidity behavioral equivalence tests");

        assert!(
            output.status.success(),
            "forge failed to compile PrivateTx.sol\nstdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );

        decode_hex_bytecode(String::from_utf8_lossy(&output.stdout).trim())
    }

    fn decode_hex_bytecode(hex: &str) -> Vec<u8> {
        let hex = hex
            .strip_prefix("0x")
            .expect("forge deployedBytecode output should be 0x-prefixed");
        assert!(
            hex.len() % 2 == 0,
            "forge deployedBytecode output should have an even number of hex digits"
        );
        (0..hex.len())
            .step_by(2)
            .map(|i| {
                u8::from_str_radix(&hex[i..i + 2], 16)
                    .expect("forge deployedBytecode output should be valid hex")
            })
            .collect()
    }

    fn branch_path_index_offset(branch: usize) -> usize {
        DEPTH_BYTES
            + NOTE_COUNT * NOTE_BYTES
            + branch * (BRANCH_HEADER_BYTES + PRIVATE_TX_DEPTH * 8)
            + 8
    }

    fn count_opcode(bytecode: &[u8], opcode: u8) -> usize {
        let mut count = 0;
        let mut offset = 0;
        while offset < bytecode.len() {
            let current = bytecode[offset];
            if current == opcode {
                count += 1;
            }
            offset += if (0x60..=0x7f).contains(&current) {
                usize::from(current - 0x5f) + 1
            } else {
                1
            };
        }
        count
    }
}
