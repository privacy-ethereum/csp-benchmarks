use clap::{Parser, Subcommand};
use hex::ToHex;
use utils::BenchTarget;
use utils::targeted::ByteHashKind;

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

    /// Generate inputs for poseidon2: prints hex-encoded input bytes then hex Poseidon2 hash
    Poseidon2 {
        /// Number of field elements (default 2)
        #[arg(long, short = 'n', default_value_t = 2)]
        size: usize,
    },

    /// Generate inputs for private_tx: prints hex-encoded input bytes then public output
    PrivateTx {
        /// Merkle branch depth (default 32)
        #[arg(long, short = 'd', default_value_t = 32)]
        depth: usize,
    },

    /// Generate inputs for constant_overhead: prints hex input then public output
    ConstantOverhead,

    /// Generate inputs for hash_sha256: prints hex input then expected folded digest
    HashSha256 {
        /// Number of 2-to-1 hashes (default 128)
        #[arg(long, short = 'n', default_value_t = 128)]
        count: usize,
    },

    /// Generate inputs for hash_keccak: prints hex input then expected folded digest
    HashKeccak {
        /// Number of 2-to-1 hashes (default 128)
        #[arg(long, short = 'n', default_value_t = 128)]
        count: usize,
    },

    /// Generate inputs for hash_blake3: prints hex input then expected folded digest
    HashBlake3 {
        /// Number of 2-to-1 hashes (default 128)
        #[arg(long, short = 'n', default_value_t = 128)]
        count: usize,
    },

    /// Generate inputs for merkle_fake: prints hex input then expected folded roots
    MerkleFake {
        /// Number of depth-32 Merkle branches (default 4)
        #[arg(long, short = 'n', default_value_t = 4)]
        branches: usize,
    },

    /// Generate inputs for merkle_sha256: prints hex input then expected folded roots
    MerkleSha256 {
        /// Number of depth-32 Merkle branches (default 4)
        #[arg(long, short = 'n', default_value_t = 4)]
        branches: usize,
    },

    /// Generate inputs for merkle_keccak: prints hex input then expected folded roots
    MerkleKeccak {
        /// Number of depth-32 Merkle branches (default 4)
        #[arg(long, short = 'n', default_value_t = 4)]
        branches: usize,
    },

    /// Generate inputs for merkle_blake3: prints hex input then expected folded roots
    MerkleBlake3 {
        /// Number of depth-32 Merkle branches (default 4)
        #[arg(long, short = 'n', default_value_t = 4)]
        branches: usize,
    },

    /// Query available input sizes from metadata
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
        Command::Poseidon2 { size } => {
            let (input_bytes, digest) = utils::generate_poseidon2_input(size);
            println!("{}", input_bytes.encode_hex::<String>());
            println!("{}", digest.encode_hex::<String>());
        }
        Command::PrivateTx { depth } => {
            let (input_bytes, public_output) = utils::generate_private_tx_input(depth);
            println!("{}", input_bytes.encode_hex::<String>());
            println!("{}", public_output.encode_hex::<String>());
        }
        Command::ConstantOverhead => {
            let (input_bytes, public_output) = utils::targeted::generate_constant_overhead_input();
            println!("{}", input_bytes.encode_hex::<String>());
            println!("{}", public_output.encode_hex::<String>());
        }
        Command::HashSha256 { count } => {
            print_hash_count(ByteHashKind::Sha256, count);
        }
        Command::HashKeccak { count } => {
            print_hash_count(ByteHashKind::Keccak, count);
        }
        Command::HashBlake3 { count } => {
            print_hash_count(ByteHashKind::Blake3, count);
        }
        Command::MerkleFake { branches } => {
            let (input_bytes, public_output) =
                utils::targeted::generate_fake_merkle_input(branches);
            println!("{}", input_bytes.encode_hex::<String>());
            println!("{}", public_output.encode_hex::<String>());
        }
        Command::MerkleSha256 { branches } => {
            print_real_merkle(ByteHashKind::Sha256, branches);
        }
        Command::MerkleKeccak { branches } => {
            print_real_merkle(ByteHashKind::Keccak, branches);
        }
        Command::MerkleBlake3 { branches } => {
            print_real_merkle(ByteHashKind::Blake3, branches);
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

fn print_hash_count(kind: ByteHashKind, count: usize) {
    let (input_bytes, public_output) = utils::targeted::generate_hash_count_input(kind, count);
    println!("{}", input_bytes.encode_hex::<String>());
    println!("{}", public_output.encode_hex::<String>());
}

fn print_real_merkle(kind: ByteHashKind, branches: usize) {
    let (input_bytes, public_output) = utils::targeted::generate_real_merkle_input(kind, branches);
    println!("{}", input_bytes.encode_hex::<String>());
    println!("{}", public_output.encode_hex::<String>());
}
