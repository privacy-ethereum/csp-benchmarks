use circom::prepare;
use circom_prover::prover::CircomProof;
use utils::harness::ProvingSystem;

utils::define_benchmark_harness!(
    BenchTarget::Sha256,
    ProvingSystem::Circom,
    None,
    "sha256_mem_circom",
    |input_size| { prepare(input_size) },
    |(witness_fn, input_str, zkey_path)| {
        circom::prove(*witness_fn, input_str.clone(), zkey_path.clone())
    },
    |(_witness_fn, _input_str, zkey_path), proof| {
        circom::verify(proof.clone(), zkey_path.clone())
    },
    |(_witness_fn, _input_str, zkey_path)| {
        // NOTE: We assume that the dir which includes "[circuit].zkey" also contains the files
        //       needed for witness generation("[circuit].cpp", "[circuit].dat" files).
        sum_file_sizes_in_the_dir(zkey_path).expect("Unable to compute preprocessing size")
    },
    |_proof: &CircomProof| 807 // NOTE: Assume that protocol is "rapidsnark" and curve is "bn128"
);

fn sum_file_sizes_in_the_dir(file_path: &str) -> std::io::Result<usize> {
    // Get the parent directory
    let dir = std::path::Path::new(file_path)
        .parent()
        .expect("File should have a parent directory");

    // Sum file sizes in that directory
    let mut total_size: usize = 0;

    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let metadata = entry.metadata()?;

        if metadata.is_file() {
            total_size += metadata.len() as usize;
        }
    }

    Ok(total_size)
}
