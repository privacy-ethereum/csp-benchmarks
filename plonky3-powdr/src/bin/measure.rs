use sha::bench::{prepare_pipeline, prove, verify};
#[cfg(not(target_env = "msvc"))]
use tikv_jemallocator::Jemalloc;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

use memory_stats::memory_stats;

fn main() {
    let usage_before = memory_stats().unwrap();

    let mut pipeline = prepare_pipeline();

    let usage_after = memory_stats().unwrap();
    println!(
        "Preprocessing memory usage: {} GB resident | {} GB virt",
        (usage_after.physical_mem - usage_before.physical_mem) as f32 / (1024.0 * 1024.0 * 1024.0),
        (usage_after.virtual_mem - usage_before.virtual_mem) as f32 / (1024.0 * 1024.0 * 1024.0)
    );

    let usage_before = memory_stats().unwrap();

    prove(&mut pipeline);

    let usage_after = memory_stats().unwrap();
    println!(
        "Proving memory usage: {} GB resident | {} GB virt",
        (usage_after.physical_mem - usage_before.physical_mem) as f32 / (1024.0 * 1024.0 * 1024.0),
        (usage_after.virtual_mem - usage_before.virtual_mem) as f32 / (1024.0 * 1024.0 * 1024.0)
    );

    verify(pipeline);
}
