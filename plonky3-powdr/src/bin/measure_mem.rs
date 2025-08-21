use sha::bench::{prepare_pipeline, prove};

fn main() {
    let mut pipeline = prepare_pipeline();

    prove(&mut pipeline);
}
