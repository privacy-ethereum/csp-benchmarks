//! SHA256 benchmark module.

use crate::{
    benchmark::Benchmark,
    traits::{BenchmarkConfig, DataGenerator, InputBuilder, Program, ZkVMBuilder},
};
use ere_jolt::EreJolt;
use ere_miden::EreMiden;
use ere_risc0::EreRisc0;
use ere_sp1::EreSP1;
use rand::{RngCore, SeedableRng, rngs::StdRng};
use utils::{
    bench::{measure_peak_memory, write_csv},
    metadata::SHA2_INPUTS,
};
use zkvm_interface::{Compiler, Input, zkVM};

/// SHA256
pub struct Sha256;
impl Program for Sha256 {
    const NAME: &'static str = "sha256";
}

/// SHA256 benchmark configuration
#[derive(Debug, Clone, Copy)]
pub struct Sha256Config {
    pub input_size: usize,
}

impl Sha256Config {
    pub fn new(input_size: usize) -> Self {
        Self { input_size }
    }
}

impl BenchmarkConfig for Sha256Config {}

/// SHA256 input data generator
pub struct Sha256Generator;

impl DataGenerator<Sha256Config> for Sha256Generator {
    type Data = Vec<u8>;

    fn generate(&self, config: &Sha256Config) -> (Self::Data, usize) {
        let mut rng = StdRng::seed_from_u64(1337);
        let mut data = vec![0; config.input_size];
        rng.fill_bytes(&mut data);
        (data, config.input_size)
    }
}

// Default input builder
macro_rules! impl_byte_vec_input_builder {
    ($zkvm:ty) => {
        impl InputBuilder<Sha256> for $zkvm {
            type Data = Vec<u8>;

            fn build_input(data: Self::Data) -> Input {
                let mut input = Input::new();
                input.write(data);
                input
            }
        }
    };
}

impl_byte_vec_input_builder!(EreRisc0);
impl_byte_vec_input_builder!(EreSP1);
impl_byte_vec_input_builder!(EreJolt);

impl InputBuilder<Sha256> for EreMiden {
    type Data = Vec<u8>;

    fn build_input(data: Self::Data) -> Input {
        let mut input = Input::new();
        let len = data.len();

        input.write_bytes((len as u64).to_le_bytes().to_vec());

        let blocks = len.div_ceil(16);
        let words_needed = blocks * 4;

        let mut words: Vec<u32> = data
            .chunks(4)
            .map(|chunk| {
                let mut bytes = [0u8; 4];
                bytes[..chunk.len()].copy_from_slice(chunk);
                u32::from_be_bytes(bytes)
            })
            .collect();

        words.resize(words_needed, 0);

        for block in words.chunks_exact(4) {
            for &word in block.iter().rev() {
                input.write_bytes((word as u64).to_le_bytes().to_vec());
            }
        }

        input
    }
}

/// Runs the SHA256 benchmark for the given zkVM.
pub fn sha256_benchmark<C, V, B>(compiler: &C, vm_builder: &B, vm_name: &'static str)
where
    C: Compiler,
    V: zkVM + InputBuilder<Sha256, Data = Vec<u8>>,
    B: ZkVMBuilder<C, V>,
{
    let configs = SHA2_INPUTS.map(Sha256Config::new);
    let benchmark = Benchmark::new(compiler, vm_name, Sha256::NAME, vm_builder).unwrap();

    let mut results = Vec::new();
    for &config in &configs {
        let (mut benchmark_result, peak_memory) = measure_peak_memory(|| {
            benchmark
                .bench::<Sha256, _, _>(&Sha256Generator, &config)
                .unwrap()
        });

        benchmark_result.1.peak_memory = peak_memory;
        results.push(benchmark_result.1);
    }

    let file_name = format!("{}-sha256.csv", vm_name);
    write_csv(&file_name, &results);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::programs::MidenBuilder;
    use ere_miden::MIDEN_TARGET;
    use sha2::{Digest, Sha256};

    #[test]
    fn test_miden() {
        let config = Sha256Config::new(2048);
        let benchmark = Benchmark::new(&MIDEN_TARGET, "miden", "sha256", &MidenBuilder).unwrap();

        let (_, _, raw_output) = benchmark.execute(&Sha256Generator, &config).unwrap();

        let (data, _) = Sha256Generator.generate(&config);
        let expected = Sha256::digest(&data);

        let parsed_output: Vec<u8> = raw_output
            .chunks_exact(8)
            .skip(1) // skip stack length
            .take(8) // 8 digest u32s
            .map(|chunk| u64::from_le_bytes(chunk.try_into().unwrap()) as u32)
            .flat_map(|word| word.to_be_bytes())
            .collect();

        assert_eq!(parsed_output, expected.as_slice());
    }
}
