use clap::{Parser, Subcommand};
use hex::ToHex;
use rand::{Rng, distributions::Uniform};
use sha2::{Digest, Sha256};

/// CLI to generate benchmark inputs and query available sizes
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Generate inputs for sha256: prints ASCII message then hex digest
    Sha256 {
        /// Input size in bytes (default 15 to match existing non-Rust example)
        #[arg(long, short = 'n', default_value_t = 15)]
        size: usize,
    },

    /// Query available sha256 input sizes from metadata
    Sizes {
        #[command(subcommand)]
        command: SizesCommand,
    },
}

#[derive(Subcommand, Debug)]
enum SizesCommand {
    /// Print JSON array of sizes (e.g., [2048])
    List,
    /// Print the number of sizes
    Len,
    /// Print the size at the given zero-based index
    Get {
        #[arg(long)]
        index: usize,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Sha256 { size } => {
            let alphabet: Vec<char> = (b'a'..=b'z').map(char::from).collect();
            let dist = Uniform::from(0..alphabet.len());
            let mut rng = rand::thread_rng();
            let mut message = String::with_capacity(size);
            for _ in 0..size {
                let idx = rng.sample(&dist);
                message.push(alphabet[idx]);
            }

            let mut hasher = Sha256::new();
            hasher.update(message.as_bytes());
            let digest = hasher.finalize().to_vec();

            println!("{}", message);
            println!("{}", digest.encode_hex::<String>());
        }
        Command::Sizes {
            command: SizesCommand::List,
        } => {
            let json =
                serde_json::to_string(&utils::metadata::SHA2_INPUTS).expect("serialize sizes");
            println!("{}", json);
        }
        Command::Sizes {
            command: SizesCommand::Len,
        } => {
            println!("{}", utils::metadata::SHA2_INPUTS.len());
        }
        Command::Sizes {
            command: SizesCommand::Get { index },
        } => {
            let sizes = &utils::metadata::SHA2_INPUTS;
            if let Some(size) = sizes.get(index) {
                println!("{}", size);
            } else {
                eprintln!("index out of range: {} (len={})", index, sizes.len());
                std::process::exit(2);
            }
        }
    }
}
