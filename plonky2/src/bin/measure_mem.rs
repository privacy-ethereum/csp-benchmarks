use std::process::Command;
use utils::metadata::SHA2_INPUTS;

fn main() {
    let script = "../measure_mem_avg.sh";
    let binary_name = "sha256_no_lookup_mem";
    for input_size in SHA2_INPUTS {
        let json_file = format!("sha256_{}_plonky2_no_lookup_mem_report.json", input_size);
        let binary_path = format!("../target/release/{}", binary_name);
        let _compile_output = Command::new("cargo")
            .arg("build")
            .arg("--release")
            .arg("--bin")
            .arg(&binary_name)
            .output()
            .expect("failed to compile");

        let output = Command::new("sh")
            .arg(script)
            .arg("--json")
            .arg(json_file)
            .arg("--")
            .arg(binary_path)
            .arg("--input-size")
            .arg(input_size.to_string())
            .output()
            .expect("failed to execute script");

        println!("{}", String::from_utf8_lossy(&output.stdout));
    }
}
