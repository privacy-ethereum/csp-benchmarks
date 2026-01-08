use clap::{Parser, Subcommand};
use hex::ToHex;
use utils::BenchTarget;

/// CLI to generate benchmark inputs and query available sizes
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Generate inputs for sha256: prints hex-encoded message bytes then hex digest
    Sha256 {
        /// Input size in bytes (default 128)
        #[arg(long, short = 'n', default_value_t = 128)]
        size: usize,
    },

    /// Generate inputs for keccak256: prints hex-encoded message bytes then hex digest
    Keccak {
        /// Input size in bytes (default 128)
        #[arg(long, short = 'n', default_value_t = 128)]
        size: usize,
    },

    /// Generate inputs for ecdsa: prints hex-encoded hashed message, public key, and signature
    Ecdsa,

    /// Generate inputs for poseidon: prints field elements as decimal strings (one per line)
    Poseidon {
        /// Number of field elements (default 2)
        #[arg(long, short = 'n', default_value_t = 2)]
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
    List {
        #[arg(long)]
        target: BenchTarget,
    },
    /// Print the number of sizes
    Len {
        #[arg(long)]
        target: BenchTarget,
    },
    /// Print the size at the given zero-based index
    Get {
        #[arg(long)]
        target: BenchTarget,
        #[arg(long)]
        index: usize,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Sha256 { size } => {
            let (message_bytes, digest) = utils::generate_sha256_input(size);
            println!("{}", message_bytes.encode_hex::<String>());
            println!("{}", digest.encode_hex::<String>());
        }
        Command::Keccak { size } => {
            let (message_bytes, digest) = utils::generate_keccak_input(size);
            println!("{}", message_bytes.encode_hex::<String>());
            println!("{}", digest.encode_hex::<String>());
        }
        Command::Ecdsa => {
            let (digest, (pub_key_x, pub_key_y), signature) = utils::generate_ecdsa_input();
            println!("{}", digest.encode_hex::<String>());
            println!("{}", pub_key_x.encode_hex::<String>());
            println!("{}", pub_key_y.encode_hex::<String>());
            println!("{}", signature.encode_hex::<String>());
        }
        Command::Poseidon { size } => {
            let field_elements = utils::generate_poseidon_input_strings(size);
            for elem in field_elements {
                println!("{}", elem);
            }
        }
        Command::Sizes {
            command: SizesCommand::List { target },
        } => {
            let json =
                serde_json::to_string(&utils::input_sizes_for(target)).expect("serialize sizes");
            println!("{}", json);
        }
        Command::Sizes {
            command: SizesCommand::Len { target },
        } => {
            println!("{}", utils::input_sizes_for(target).len());
        }
        Command::Sizes {
            command: SizesCommand::Get { target, index },
        } => {
            let sizes = &utils::input_sizes_for(target);
            if let Some(size) = sizes.get(index) {
                println!("{}", size);
            } else {
                eprintln!("index out of range: {} (len={})", index, sizes.len());
                std::process::exit(2);
            }
        }
    }
}
