use bincode::Options;
use std::path::Path;
use zkvm_interface::Compiler;

/// Holds a compiled program together with its serialized size.
pub struct CompiledProgram<C: Compiler> {
    pub program: C::Program,
    pub byte_size: usize,
}

/// Compiles a guest program located at `guest_dir` and tracks its serialized size.
pub fn compile_guest_program<C: Compiler>(
    compiler: &C,
    guest_dir: &Path,
) -> Result<CompiledProgram<C>, C::Error> {
    let program = compiler.compile(guest_dir)?;
    let cfg = bincode::options();
    let byte_size = cfg
        .serialize(&program)
        .map(|bytes| bytes.len())
        .unwrap_or_default();
    Ok(CompiledProgram { program, byte_size })
}
