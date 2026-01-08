const SHA2_INPUTS_REDUCED: [usize; 2] = [128, 256];
const SHA2_INPUTS_FULL: [usize; 5] = [128, 256, 512, 1024, 2048];

pub fn selected_sha2_inputs() -> Vec<usize> {
    match std::env::var("BENCH_INPUT_PROFILE").ok().as_deref() {
        Some("reduced") => SHA2_INPUTS_REDUCED.to_vec(),
        _ => SHA2_INPUTS_FULL.to_vec(),
    }
}

const POSEIDON_INPUTS_REDUCED: [usize; 2] = [2, 8];
const POSEIDON_INPUTS_FULL: [usize; 5] = [2, 4, 8, 12, 16];

pub fn selected_poseidon_inputs() -> Vec<usize> {
    match std::env::var("BENCH_INPUT_PROFILE").ok().as_deref() {
        Some("reduced") => POSEIDON_INPUTS_REDUCED.to_vec(),
        _ => POSEIDON_INPUTS_FULL.to_vec(),
    }
}
