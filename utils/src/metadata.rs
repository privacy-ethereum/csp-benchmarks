const BYTE_INPUTS_REDUCED: [usize; 2] = [128, 256];
const BYTE_INPUTS_FULL: [usize; 5] = [128, 256, 512, 1024, 2048];

pub fn selected_byte_inputs() -> Vec<usize> {
    match std::env::var("BENCH_INPUT_PROFILE").ok().as_deref() {
        Some("reduced") => BYTE_INPUTS_REDUCED.to_vec(),
        _ => BYTE_INPUTS_FULL.to_vec(),
    }
}

const FIELD_ELEMENT_INPUTS_REDUCED: [usize; 2] = [2, 8];
const FIELD_ELEMENT_INPUTS_FULL: [usize; 5] = [2, 4, 8, 12, 16];

pub fn selected_field_element_inputs() -> Vec<usize> {
    match std::env::var("BENCH_INPUT_PROFILE").ok().as_deref() {
        Some("reduced") => FIELD_ELEMENT_INPUTS_REDUCED.to_vec(),
        _ => FIELD_ELEMENT_INPUTS_FULL.to_vec(),
    }
}

const PRIVATE_TX_DEPTHS_REDUCED: [usize; 1] = [32];
const PRIVATE_TX_DEPTHS_FULL: [usize; 1] = [32];

pub fn selected_private_tx_depths() -> Vec<usize> {
    match std::env::var("BENCH_INPUT_PROFILE").ok().as_deref() {
        Some("reduced") => PRIVATE_TX_DEPTHS_REDUCED.to_vec(),
        _ => PRIVATE_TX_DEPTHS_FULL.to_vec(),
    }
}

const HASH_COUNTS_REDUCED: [usize; 2] = [128, 2048];
const HASH_COUNTS_FULL: [usize; 2] = [128, 2048];

pub fn selected_hash_counts() -> Vec<usize> {
    match std::env::var("BENCH_INPUT_PROFILE").ok().as_deref() {
        Some("reduced") => HASH_COUNTS_REDUCED.to_vec(),
        _ => HASH_COUNTS_FULL.to_vec(),
    }
}

const MERKLE_BRANCH_COUNTS_REDUCED: [usize; 2] = [4, 32];
const MERKLE_BRANCH_COUNTS_FULL: [usize; 2] = [4, 32];

pub fn selected_merkle_branch_counts() -> Vec<usize> {
    match std::env::var("BENCH_INPUT_PROFILE").ok().as_deref() {
        Some("reduced") => MERKLE_BRANCH_COUNTS_REDUCED.to_vec(),
        _ => MERKLE_BRANCH_COUNTS_FULL.to_vec(),
    }
}

pub fn selected_constant_overhead_inputs() -> Vec<usize> {
    vec![1]
}
