use utils::{
    bench::{compile_binary, run_measure_mem_script},
    metadata::SHA2_INPUTS,
};

fn main() {
    let sha256_binary_name = "sha256_mem";
    for input_size in SHA2_INPUTS {
        compile_binary(sha256_binary_name);

        let sha256_binary_path = format!("../target/release/{}", sha256_binary_name);
        let json_file = format!("sha256_{}_powdr_mem_report.json", input_size);
        run_measure_mem_script(&json_file, &sha256_binary_path, input_size);
    }
}
