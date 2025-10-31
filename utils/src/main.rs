use clap::{Parser, Subcommand};
use hex::ToHex;

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

    /// Generate inputs for ecdsa: prints hex-encoded hashed message, public key, and signature
    Ecdsa,

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
            let (message_bytes, digest) = utils::generate_sha256_input(size);
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
        Command::Sizes {
            command: SizesCommand::List,
        } => {
            let json = serde_json::to_string(&utils::metadata::selected_sha2_inputs())
                .expect("serialize sizes");
            println!("{}", json);
        }
        Command::Sizes {
            command: SizesCommand::Len,
        } => {
            println!("{}", utils::metadata::selected_sha2_inputs().len());
        }
        Command::Sizes {
            command: SizesCommand::Get { index },
        } => {
            let sizes = &utils::metadata::selected_sha2_inputs();
            if let Some(size) = sizes.get(index) {
                println!("{}", size);
            } else {
                eprintln!("index out of range: {} (len={})", index, sizes.len());
                std::process::exit(2);
            }
        }
    }
}
