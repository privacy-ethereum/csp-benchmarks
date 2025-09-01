use rand::{RngCore, SeedableRng, rngs::StdRng};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use serde::Serialize;

pub mod bench;
pub mod metadata;

pub fn sha2_input(num_bytes: usize) -> Vec<u8> {
    let mut rng = StdRng::seed_from_u64(1337);
    let mut message = vec![0; num_bytes];
    rng.fill_bytes(&mut message);
    message
}

pub fn load_elf(path: &str) -> Vec<u8> {
    fs::read(path).unwrap_or_else(|err| {
        panic!("Failed to load ELF file from {}: {}", path, err);
    })
}

pub fn size<T: Serialize>(item: &T) -> usize {
    bincode::serialized_size(item).unwrap() as usize
}

pub fn write_json<T: Serialize>(data: &T, output_path: &str) {
    let json_data = serde_json::to_string_pretty(&data).expect("Failed to serialize to JSON");
    let path = Path::new(&output_path);

    let mut file = File::create(path).expect("Failed to create file");
    file.write_all(json_data.as_bytes())
        .expect("Failed to write to file");
}
