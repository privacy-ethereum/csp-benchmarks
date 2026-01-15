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
