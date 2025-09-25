//! SHA256 benchmark module.

use crate::{
    traits::{BenchmarkConfig, DataGenerator, InputBuilder, Program},
    zkvm::SupportedVms,
};
use ere_jolt::EreJolt;
use ere_miden::EreMiden;
use ere_risc0::EreRisc0;
use ere_sp1::EreSP1;
use rand::{RngCore, SeedableRng, rngs::StdRng};
use zkvm_interface::Input;

/// RNG seed for SHA256 input generation.
pub const RNG_SEED: u64 = 1337;

/// Supported SHA256 configurations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SupportedConfigs {
    Size2048,
}

impl TryFrom<&str> for SupportedConfigs {
    type Error = &'static str;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "2048" => Ok(SupportedConfigs::Size2048),
            _ => Err("Unknown config"),
        }
    }
}

impl ToString for SupportedConfigs {
    fn to_string(&self) -> String {
        match self {
            SupportedConfigs::Size2048 => "2048".to_string(),
        }
    }
}

/// SHA256 program marker.
pub struct Sha256;

impl Program for Sha256 {
    const NAME: &'static str = "sha256";
}

/// SHA256 benchmark configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Sha256Config {
    pub input_size: usize,
}

impl Sha256Config {
    pub const fn new(input_size: usize) -> Self {
        Self { input_size }
    }
}

impl BenchmarkConfig for Sha256Config {}

impl From<SupportedConfigs> for Sha256Config {
    fn from(cfg: SupportedConfigs) -> Self {
        match cfg {
            SupportedConfigs::Size2048 => Sha256Config::new(2048),
        }
    }
}

/// Random input generator for SHA256.
pub struct Sha256Generator;
impl DataGenerator<Sha256Config> for Sha256Generator {
    type Data = Vec<u8>;

    fn generate(&self, cfg: &Sha256Config) -> (Self::Data, usize) {
        let mut rng = StdRng::seed_from_u64(RNG_SEED);
        let mut data = vec![0; cfg.input_size];
        rng.fill_bytes(&mut data);
        (data, cfg.input_size)
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

// Miden custom input builder.
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

/// Build a Sha256 input for a given VM.
pub fn build_input(vm: &SupportedVms, data: &[u8]) -> Input {
    match vm {
        SupportedVms::Risc0 => <EreRisc0 as InputBuilder<Sha256>>::build_input(data.to_vec()),
        SupportedVms::Sp1 => <EreSP1 as InputBuilder<Sha256>>::build_input(data.to_vec()),
        SupportedVms::Jolt => <EreJolt as InputBuilder<Sha256>>::build_input(data.to_vec()),
        SupportedVms::Miden => <EreMiden as InputBuilder<Sha256>>::build_input(data.to_vec()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{programs::MidenBuilder, zkvm::ZkVMInstance};
    use ere_miden::MIDEN_TARGET;
    use sha2::{Digest, Sha256};

    #[test]
    fn miden_sha256_matches_reference() {
        let zkvm = ZkVMInstance::new(&MIDEN_TARGET, "miden", "sha256", &MidenBuilder).unwrap();

        let cfg = Sha256Config::new(2048);
        let (data, _) = Sha256Generator.generate(&cfg);
        let input = EreMiden::build_input(data.clone());

        let (raw_output, _) = zkvm.execute_only(&input).unwrap();

        let (data, _) = Sha256Generator.generate(&cfg);
        let expected = Sha256::digest(&data);

        let parsed: Vec<u8> = raw_output
            .chunks_exact(8)
            .skip(1)
            .take(8)
            .map(|c| u64::from_le_bytes(c.try_into().unwrap()) as u32)
            .flat_map(|w| w.to_be_bytes())
            .collect();

        assert_eq!(parsed, expected.as_slice());
    }
}
