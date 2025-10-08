use sha::bench::{prepare_pipeline, prove, verify};
use utils::harness::ProvingSystem;

utils::define_benchmark_harness!(
    sha256,
    ProvingSystem::Powdr,
    None,
    Some("sha256_mem_powdr"),
    |input_size| { prepare_pipeline(input_size) },
    |pipeline: &mut sha::bench::Pipeline| { prove(pipeline) },
    |pipeline: &mut sha::bench::Pipeline, _| { verify(pipeline) },
    |pipeline: sha::bench::Pipeline| {
        let pk_bytes = std::fs::read("powdr-target/pkey.bin").expect("Unable to read file");
        let constants_bytes =
            std::fs::read("powdr-target/constants.bin").expect("Unable to read file");
        let pil_bytes = std::fs::read("powdr-target/guest.pil").expect("Unable to read file");
        pk_bytes.len() + constants_bytes.len() + pil_bytes.len()
    },
    |proof: &Vec<u8>| { proof.len() }
);
