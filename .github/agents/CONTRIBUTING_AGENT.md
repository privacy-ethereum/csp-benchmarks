# Claude Agent Instructions for CSP Benchmarks Repository

## Repository Overview

This is the Client-Side Proving (CSP) Benchmarks repository, which contains benchmarks of zero-knowledge proving systems and zkVMs. The goal is to continuously map the landscape of proving systems suitable for client-side environments.

**Key Facts:**
- Repository path: `/home/runner/work/csp-benchmarks/csp-benchmarks`
- Primary language: Rust (with some non-Rust systems like Noir, C++)
- Benchmarks run quarterly on Apple M1 (arm64) hardware
- Results published at https://ethproofs.org/csp-benchmarks
- Canonical circuits: SHA-256, ECDSA, Keccak, etc.

## Repository Structure

```
/home/runner/work/csp-benchmarks/csp-benchmarks/
├── utils/              # Shared benchmark harness and metadata
├── benchmark.sh        # Orchestration for non-Rust systems
├── measure_mem_avg.sh  # RAM measurement helper
├── results/            # Published benchmark results
├── CONTRIBUTING.md     # Detailed contributor guide
├── README.MD           # Project overview
├── Cargo.toml          # Workspace root
├── rust-toolchain.toml # Rust toolchain specification
├── [system-name]/      # Individual proving system benchmarks
│   ├── benches/        # (Rust) Criterion benchmarks
│   ├── src/            # (Rust) Source code
│   ├── bin/            # (Rust) Memory measurement binaries
│   ├── [target]_*.sh   # (non-Rust) Shell scripts per target
│   ├── bench_props.json # (non-Rust) Metadata
│   └── circuit_sizes.json # (non-Rust) Constraint counts
└── mobile/             # Mobile benchmarks
```

## Before Making Changes

### 1. Always Read CONTRIBUTING.md First
The file at `/home/runner/work/csp-benchmarks/csp-benchmarks/CONTRIBUTING.md` contains the authoritative contributor guide. Read it carefully before making any changes.

### 2. Understand the Codebase
- Run `cargo build --release --workspace` to understand the build process
- Check existing benchmarks in folders like `plonky2/`, `sp1/`, `risc0/`, etc.
- Look at `utils/src/harness.rs` to understand the benchmark harness API
- Review `utils/src/metadata.rs` for canonical input sizes

### 3. Determine the Type of Contribution
- **Rust benchmark**: Use the `utils::define_benchmark_harness!` macro
- **Non-Rust benchmark**: Implement shell scripts and `bench_props.json`

## Contributing Rust Benchmarks

### Required Steps

1. **Create a top-level directory** for your proving system (e.g., `my-system/`)

2. **Add to workspace** in root `Cargo.toml`:
   ```toml
   members = [
       # ... existing members ...
       "my-system",
   ]
   ```

3. **Use the shared harness** from the `utils` crate. Two patterns:

   **Pattern A: No shared state**
   ```rust
   use utils::harness::{BenchTarget, ProvingSystem, BenchProperties, AuditStatus};
   use std::borrow::Cow;
   
   const MY_SYSTEM_BENCH_PROPERTIES: BenchProperties = BenchProperties {
       proving_system: Cow::Borrowed("MySystem"),
       field_curve: Cow::Borrowed("BN254"),
       iop: Cow::Borrowed("Groth16"),
       pcs: None,
       arithm: Cow::Borrowed("R1CS"),
       is_zk: true,
       is_zkvm: false,
       security_bits: 128,
       is_pq: false,
       is_maintained: true,
       is_audited: AuditStatus::NotAudited,
       isa: None,
   };
   
   utils::define_benchmark_harness!(
       BenchTarget::Sha256,            // target
       ProvingSystem::MySystem,        // proving system
       None,                           // optional feature tag
       "sha256_mem_mysystem",         // memory binary name
       MY_SYSTEM_BENCH_PROPERTIES,    // BenchProperties
       |input_size| { /* prepare */ },
       |prepared| { /* num_constraints */ 0 },
       |prepared| { /* prove */ },
       |prepared, proof| { /* verify */ },
       |prepared| { /* preprocessing_size */ 0 },
       |proof| { /* proof_size */ 0 }
   );
   ```

   **Pattern B: With shared state**
   ```rust
   use utils::harness::{BenchTarget, ProvingSystem, BenchProperties};
   
   utils::define_benchmark_harness!(
       BenchTarget::Sha256,
       ProvingSystem::MySystem,
       None,
       "sha256_mem_mysystem",
       MY_SYSTEM_BENCH_PROPERTIES,    // BenchProperties
       { /* initialize shared state once */ },
       |size, shared| { /* prepare with shared */ },
       |prepared, shared| { /* num_constraints with shared */ 0 },
       |prepared, shared| { /* prove with shared */ },
       |prepared, proof, shared| { /* verify with shared */ },
       |prepared, shared| { /* preprocessing_size */ 0 },
       |proof, shared| { /* proof_size */ 0 }
   );
   ```

4. **Provide a memory measurement binary** (in `bin/` or `src/bin/`) that:
   - Takes `INPUT_SIZE` environment variable
   - Performs circuit preprocessing and proving (including witness generation)
   - Exits cleanly
   - Name matches the `mem_binary_name` parameter

5. **Format and lint** before committing:
   ```bash
   cargo fmt
   cargo clippy
   ```

### Input Sizes
- Variable-size targets (SHA-256, Keccak): use pre-defined sizes from `utils::metadata`
- Fixed-size targets (ECDSA): use a single input size value
- The harness automatically iterates over appropriate sizes based on `BENCH_INPUT_PROFILE`

### Testing Your Benchmark
```bash
# Quick test with reduced inputs
BENCH_INPUT_PROFILE=reduced cargo bench -p my-system

# Full benchmark
BENCH_INPUT_PROFILE=full cargo bench -p my-system
```

## Contributing Non-Rust Benchmarks

### Required Steps

1. **Create a top-level folder** named after your system (e.g., `my-system/`)

2. **Register in CI**: Add your folder to `.github/workflows/sh_benchmarks_parallel.yml`:
   ```yaml
   FOLDERS: >-
     [
       "barretenberg",
       "ligetron",
       "my-system"
     ]
   ```

3. **Create `bench_props.json`** at the folder root with metadata:
   ```json
   {
     "proving_system": "MySystem",
     "field_curve": "BN254",
     "iop": "Groth16",
     "pcs": "KZG",
     "arithm": "R1CS",
     "is_zk": true,
     "is_zkvm": false,
     "security_bits": 128,
     "is_pq": false,
     "is_maintained": true,
     "is_audited": "not_audited",
     "isa": null
   }
   ```
   See `utils/src/harness.rs` for full `BenchProperties` schema. Valid values for `is_audited`: `"audited"`, `"not_audited"`, `"partially_audited"`. For zkVMs, set `is_zkvm: true` and provide an `isa` value (e.g., `"RISC-V"`, `"WASM"`). Refer to `barretenberg/bench_props.json` and `ligetron/bench_props.json` for concrete examples.

4. **Implement 4-5 shell scripts per target**:

   #### `[target]_prepare.sh`
   - **Input**: `$UTILS_BIN`, `$INPUT_SIZE`, `$STATE_JSON`
   - **Output**: Write JSON to `$STATE_JSON` containing input state for prover/verifier
   - **Purpose**: Generate and serialize the circuit input
   
   #### `[target]_prove.sh`
   - **Input**: `$STATE_JSON`
   - **Output**: Produce proof artifact in predictable location
   - **Purpose**: Run the prover (used for timing)
   
   #### `[target]_prove_for_verify.sh` (optional)
   - **Input**: `$STATE_JSON`
   - **Output**: Proof + any verification artifacts (e.g., verification key)
   - **Purpose**: Generate proof and verification data when separate from prove
   
   #### `[target]_verify.sh`
   - **Input**: `$STATE_JSON` (and proof from previous step in CI)
   - **Output**: Exit 0 on success, non-zero on failure
   - **Purpose**: Run the verifier (used for timing)
   
   #### `[target]_measure.sh`
   - **Input**: `$STATE_JSON`, `$SIZES_JSON`
   - **Output**: 
     - Write JSON to `$SIZES_JSON`: `{"proof_size": N, "preprocessing_size": M}`
     - Update/create `circuit_sizes.json` in your folder with constraint counts
   - **Purpose**: Measure three key metrics:
     1. **Proof size**: Size of the generated proof in bytes
     2. **Preprocessing size**: Size of preprocessing artifacts (e.g., proving key/zkey) in bytes
     3. **Circuit constraints/gates**: Number of constraints or gates in the compiled circuit

5. **Make scripts executable**:
   ```bash
   chmod +x my-system/*.sh
   ```

6. **Create/update `circuit_sizes.json`** dynamically in `[target]_measure.sh`:
   
   The `[target]_measure.sh` script must compile the circuit for the given input size and measure the number of constraints in the resulting compiled circuit. This is done dynamically because non-Rust systems typically do not expose constraint counts as a dedicated API. The script should update `circuit_sizes.json` with the measured values:
   
   ```json
   {
     "sha256": {
       "128": 67890,
       "1024": 543210
     },
     "ecdsa": {
       "32": 12345
     }
   }
   ```

### Testing Non-Rust Benchmarks
```bash
# Build utils first
cargo build --release -p utils

# Quick test
BENCH_INPUT_PROFILE=full bash ./benchmark.sh \
  --system-dir ./my-system \
  --logging \
  --quick \
  --no-ram

# Full benchmark
BENCH_INPUT_PROFILE=full bash ./benchmark.sh \
  --system-dir ./my-system \
  --logging
```

## Output Files and Naming Conventions

### Rust Systems
- Metrics: `{target}_{input}_{system}_[feature]_metrics.json`
- Memory: `{target}_{input}_{system}_[feature]_mem_report.json`
- Generated automatically by the harness

### Non-Rust Systems
- Metrics: `{target}_{input}_{system}_metrics.json`
- Memory: `{target}_{input}_{system}_mem_report.json`
- Sizes: `{target}_{input}_sizes.json`
- Circuit sizes: `circuit_sizes.json` (in system folder)
- Raw hyperfine: `hyperfine_{target}_{input}_*.json`

## Common Pitfalls and Best Practices

### DO:
- ✅ Read CONTRIBUTING.md thoroughly before starting
- ✅ Use `cargo fmt --all -- --check` and `cargo clippy --workspace --all-targets --all-features` for Rust code (as defined in `.github/workflows/lints.yml`)
- ✅ Test with `BENCH_INPUT_PROFILE=reduced` first for quick iteration
- ✅ Make scripts executable with `chmod +x`
- ✅ Use the shared harness macro - don't write custom benchmark code
- ✅ Follow existing examples (plonky2, circom, sp1, barretenberg, ligetron)
- ✅ Record circuit sizes accurately
- ✅ Ensure memory binaries perform only preprocessing + proving
- ✅ Use `git --no-pager` when running git commands via bash
- ✅ Validate that your benchmarks run successfully before submitting

### DON'T:
- ❌ Skip reading CONTRIBUTING.md
- ❌ Add benchmarks without using the shared harness
- ❌ Forget to register new crates in root Cargo.toml
- ❌ Forget to add non-Rust systems to CI workflow
- ❌ Create non-executable shell scripts
- ❌ Modify the utils crate without understanding implications
- ❌ Break existing benchmarks while adding new ones
- ❌ Include build artifacts or generated files in commits
- ❌ Use interactive pagers in bash commands (always use --no-pager)

## Key Environment Variables

- `BENCH_INPUT_PROFILE`: Set to `full` or `reduced` to control input sizes
- `UTILS_BIN`: Path to utils binary (for non-Rust systems)
- `INPUT_SIZE`: Input size in bytes (for non-Rust prepare scripts)
- `STATE_JSON`: Path to state JSON (for non-Rust prove/verify/measure scripts)
- `SIZES_JSON`: Path to sizes output JSON (for non-Rust measure scripts)

## Reference Examples

### Rust Benchmarks
- **Simple**: `plonky2/benches/sha256.rs`, `circom/benches/sha256_bench.rs`
- **With shared state**: `polyhedra-expander/benches/sha256.rs`
- **zkVM**: `sp1/benches/sha256.rs`, `risc0/benches/sha256.rs`

### Non-Rust Benchmarks
- **Noir/Barretenberg**: `barretenberg/`
- **Ligero/Ligetron**: `ligetron/`

## CI and Testing

### CI Workflows
- `.github/workflows/rust_benchmarks_parallel.yml`: Rust benchmarks
- `.github/workflows/sh_benchmarks_parallel.yml`: Non-Rust benchmarks

### Pre-submit Checklist
1. Code compiles: `cargo build --workspace`
2. Linting passes: `cargo clippy --workspace --all-targets --all-features`
3. Formatting correct: `cargo fmt --all -- --check`
4. Quick benchmark runs: `BENCH_INPUT_PROFILE=reduced cargo bench`
5. Scripts are executable: `chmod +x my-system/*.sh` (for non-Rust)
6. CI workflow updated (for non-Rust): folder added to `sh_benchmarks_parallel.yml`
7. For excluded crates (cairo-m, nexus): Run commands from within their directories

## Getting Help

- **Primary guide**: `/home/runner/work/csp-benchmarks/csp-benchmarks/CONTRIBUTING.md`
- **Harness API**: `utils/src/harness.rs`
- **Metadata**: `utils/src/metadata.rs`
- **Examples**: Existing system folders (`plonky2/`, `sp1/`, `barretenberg/`, etc.)

## Agent-Specific Notes

As a Claude agent working on this repository:

1. **Always start** by reading CONTRIBUTING.md - it's the source of truth
2. **Use parallel tool calls** when exploring multiple files or running independent commands
3. **Test incrementally** - use `--quick` and `reduced` profiles during development
4. **Follow existing patterns** - look at similar systems for reference
5. **Validate thoroughly** - ensure benchmarks actually run before reporting completion
6. **Keep changes minimal** - follow the repository's established patterns and conventions
7. **Use absolute paths** - all paths should start with `/home/runner/work/csp-benchmarks/csp-benchmarks/`
8. **Check git status** frequently to avoid committing unwanted files
9. **Disable pagers** in git commands: use `git --no-pager` for all git commands
10. **Memory binaries** are critical - ensure they exist and follow naming conventions

## Security Considerations

- Check for vulnerabilities with `codeql_checker` tool (available in the agent environment) before finalizing
- Don't commit secrets or credentials
- Validate all external inputs in benchmark code
- Use safe Rust practices (avoid unwrap() in production code paths)

## Final Validation

Before reporting completion:
1. Run `cargo build --workspace` successfully
   - **Note**: Some crates (`cairo-m`, `nexus`) are excluded from the workspace. Build them separately:
     ```bash
     cd cairo-m && cargo build && cd ..
     cd nexus && cargo build && cd ..
     ```
2. Run `cargo clippy --workspace --all-targets --all-features` with no errors
   - For excluded crates: `cd cairo-m && cargo clippy --all-targets --all-features && cd ..` (and same for nexus)
3. Run `cargo fmt --all -- --check` 
4. Test benchmark with `BENCH_INPUT_PROFILE=reduced`
5. Verify output files are generated correctly
6. Check git status to ensure only intended files are staged
7. Request code review via `code_review` tool (available in the agent environment)
8. Run `codeql_checker` tool (available in the agent environment) for security validation

---

**Remember**: The goal is to maintain a consistent, high-quality benchmarking suite that accurately measures client-side proving performance across diverse systems. Every benchmark should be reproducible, well-documented, and follow the established patterns.
