#![no_main]

sp1_zkvm::entrypoint!(main);

pub fn main() {
    let input = sp1_zkvm::io::read::<Vec<u8>>();
    let output =
        evm_sp1_private_tx::execute_private_tx(&input).expect("private_tx EVM execution failed");
    sp1_zkvm::io::commit_slice(&output);
}
