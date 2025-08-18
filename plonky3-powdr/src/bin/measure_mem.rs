use sha::bench::{prepare_pipeline, prove};

fn main() {
    let mut pipeline = prepare_pipeline();

    let _proof = prove(&mut pipeline);
}
