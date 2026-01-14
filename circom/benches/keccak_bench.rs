use circom::{
    CIRCOM_BENCH_PROPERTIES,
    keccak::{prepare, prove, verify},
    proof_size, read_constraint_count, sum_file_sizes_in_the_dir,
};
use utils::harness::ProvingSystem;

utils::define_benchmark_harness!(
    BenchTarget::Keccak,
    ProvingSystem::Circom,
    None,
    "keccak_mem_circom",
    CIRCOM_BENCH_PROPERTIES,
    |input_size| { prepare(input_size) },
    |(_witness_fn, _input_str, zkey_path)| read_constraint_count(zkey_path),
    |(witness_fn, input_str, zkey_path)| {
        prove(*witness_fn, input_str.clone(), zkey_path.clone())
    },
    |(_witness_fn, _input_str, zkey_path), proof| { verify(proof.clone(), zkey_path.clone()) },
    |(_witness_fn, _input_str, zkey_path)| {
        sum_file_sizes_in_the_dir(zkey_path).expect("Unable to compute preprocessing size")
    },
    proof_size
);
