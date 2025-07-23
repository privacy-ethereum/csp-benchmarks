use anyhow::Error;
use binius::bench::{prove, sha256_no_lookup_prepare, verify};
use memory_stats::memory_stats;

fn main() -> Result<(), Error> {
    let allocator = bumpalo::Bump::new();

    let usage_before = memory_stats().unwrap();
    let (constraint_system, args, witness, backend) = sha256_no_lookup_prepare(&allocator);
    let usage_after = memory_stats().unwrap();

    println!(
        "Preprocessing memory usage: {} GB resident | {} GB virt",
        (usage_after.physical_mem - usage_before.physical_mem) as f32 / (1024.0 * 1024.0 * 1024.0),
        (usage_after.virtual_mem - usage_before.virtual_mem) as f32 / (1024.0 * 1024.0 * 1024.0)
    );

    let usage_before = memory_stats().unwrap();
    let (cs, args, proof) = prove(constraint_system, args, witness, backend);
    let usage_after = memory_stats().unwrap();
    
    println!(
        "Proving memory usage: {} GB resident | {} GB virt",
        (usage_after.physical_mem - usage_before.physical_mem) as f32 / (1024.0 * 1024.0 * 1024.0),
        (usage_after.virtual_mem - usage_before.virtual_mem) as f32 / (1024.0 * 1024.0 * 1024.0)
    );

    verify(cs, args, proof);

    Ok(())
}
