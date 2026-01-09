use circom::{
    CIRCOM_BENCH_PROPERTIES, prepare, proof_size, read_constraint_count, sum_file_sizes_in_the_dir,
};
use utils::harness::ProvingSystem;

utils::define_benchmark_harness!(
    BenchTarget::Sha256,
    ProvingSystem::Circom,
    None,
    "sha256_mem_circom",
    CIRCOM_BENCH_PROPERTIES,
    |input_size| { prepare(input_size) },
    |(_witness_fn, _input_str, zkey_path)| read_constraint_count(zkey_path),
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
    |proof| proof_size(proof)
);
