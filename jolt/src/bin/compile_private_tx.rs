use ere_jolt::compiler::RustRv64imacCustomized;
use utils::zkvm::PRIVATE_TX_BENCH;
use utils::zkvm::helpers::load_or_compile_program;

fn main() {
    println!("Compiling private_tx guest program...");
    load_or_compile_program(&RustRv64imacCustomized, PRIVATE_TX_BENCH);
    println!("Done.");
}
