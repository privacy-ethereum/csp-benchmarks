use binius::bench::{prove, sha256_no_lookup_prepare};

fn main() {
    let allocator = bumpalo::Bump::new();

    let (constraint_system, args, witness, backend) = sha256_no_lookup_prepare(&allocator);

    let (_, _, _proof) = prove(constraint_system, args, witness, backend);
}
