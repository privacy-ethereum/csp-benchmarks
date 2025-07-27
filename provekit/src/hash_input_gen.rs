use rand::RngCore;
use std::fs;
use std::io::Write;

pub fn generate_hash_inputs() -> Result<(), &'static str> {
    fs::create_dir_all("output/hash-input").map_err(|_| "Failed to create output directory")?;
    let mut rng = rand::rng();

    for exp in [8, 10, 12, 14] {
        let size = 1usize << exp;
        let bin_path = format!("output/hash-input/input_2e{}.bin", exp);

        println!("Generating input for 2^{}", exp);

        let mut data = vec![0u8; size];
        rng.fill_bytes(&mut data);

        let mut file = fs::File::create(&bin_path).map_err(|_| "Failed to create file")?;
        file.write_all(&data)
            .map_err(|_| "Failed to write to file")?;
    }

    println!("Input generation complete.");

    Ok(())
}
