# How to Contribute

Howdy! Usual good software engineering practices apply. Write comments. Follow standard Rust coding practices where possible. Use `cargo fmt` and `cargo clippy` to tidy up formatting.

## What's Expected in the Contribution/PR

We depend on 3 JSON files for collecting benchmarks of the proving systems:

- Metrics
- Criterion report
- Memory report

When you add a new benchmark for a certain proving system, you should add a benchmarking code directory that are able to generate these JSON files. The directory should be included in the workspace(root-level `Cargo.toml`).

### Rules for JSON Files

#### Naming

- Metrics file: `[target]_[size]_[proving_system]_[(optional)feat]_metrics.json`
- Memory report: `[target]_[size]_[proving_system]_[(optional)feat]_mem_report.json`
- Criterion report: `[target]_[size]_[proving_system]_[(optional)feat]_prove` and `[target]_[size]_[proving_system]_[(optional)feat]_verify`

NOTE: Criterion report files are generated under the `target/criterion` directory of workspace. Hence, you should apply above naming to criterion benchmark function IDs.

Example:

- When you want to add a benchmark for plonky2 with no-lookup for target function `sha256` with input size `2048`:
  - Metrics: `sha256_2048_plonky2_no_lookup_metrics.json`
  - Memory report: `sha256_2048_plonky2_no_lookup_mem_report.json`
  - Criterion IDs: `sha256_2048_plonky2_no_lookup_prove` and `sha256_2048_plonky2_no_lookup_verify`

You can reference the existing benchmarks for naming. (e.g. [./plonky2/benches/prove_verify.rs](https://github.com/privacy-ethereum/csp-benchmarks/blob/CSP-Q3-2025/plonky2/benches/prove_verify.rs#L14-L68))

#### Contents

- Metrics file: should be deserialized into the following struct:

```rust
#[serde_as]
#[derive(Serialize, Deserialize, Tabled, Clone)]
pub struct Metrics {
    pub name: String,
    pub feat: String,
    pub is_zkvm: bool,
    pub target: String,
    #[tabled(display_with = "display_bytes")]
    pub input_size: usize,
    #[serde_as(as = "DurationNanoSeconds")]
    #[tabled(display_with = "display_duration")]
    pub proof_duration: Duration,
    #[serde_as(as = "DurationNanoSeconds")]
    #[tabled(display_with = "display_duration")]
    pub verify_duration: Duration,
    #[tabled(display_with = "display_cycles")]
    pub cycles: u64,
    #[tabled(display_with = "display_bytes")]
    pub proof_size: usize,
    #[tabled(display_with = "display_bytes")]
    pub preprocessing_size: usize,
    #[tabled(display_with = "display_bytes")]
    pub peak_memory: usize,
}
```

Example:

```json
{
  "name": "plonky2",
  "feat": "no_lookup",
  "is_zkvm": false,
  "target": "sha256",
  "input_size": 2048,
  "proof_duration": 10644587248,
  "verify_duration": 4647043282,
  "cycles": 0,
  "proof_size": 475590,
  "preprocessing_size": 329524,
  "peak_memory": 0
}
```

You can reference how to create JSON files of metrics in the existing benchmarks.(e.g. [./plonky2/benches/prove_verify.rs](https://github.com/privacy-ethereum/csp-benchmarks/blob/CSP-Q3-2025/plonky2/benches/prove_verify.rs#L16-L20))

```json
{
  "peak_memory": 44869222
}
```

We provide the shell script to make this JSON file - `./measure_mem_avg.sh`.  
You can reference how we do the memory measurement in the existing benchmarks.(e.g. [./plonky2/benches/prove_verify.rs](https://github.com/privacy-ethereum/csp-benchmarks/blob/CSP-Q3-2025/plonky2/benches/prove_verify.rs#L22-L28))

For more details, please check the existing benchmarks like `binius64` and `plonky2`.

## Contributing a Non-Rust Benchmark

We provide a generic orchestrator at the repo root (`./benchmark.sh`) and a CI workflow that will run non-Rust systems in parallel. This section explains how to add your own non-Rust benchmark, using `ligetron` as a concrete example.

### 1) Place your system under a top-level folder

- Create a top-level folder named after your system, e.g. `ligetron/`.
- Inside it, you will provide the code necessary to prove and verify your circuits, and shell scripts per target (e.g. `sha256`), described below.

### 2) Register your folder in CI

Add your folder name to the `FOLDERS` array in the non-Rust workflow so CI will pick it up when you open a PR:

- Edit `.github/workflows/sh_benchmarks_parallel.yml`
- Add your folder to the list (example shows `ligetron`):

```61:63:.github/workflows/sh_benchmarks_parallel.yml
FOLDERS=(
            ligetron
          )
```

### 3) Implement four shell scripts per target

The orchestrator expects four scripts in your folder for each target name (e.g. `sha256`). The scripts must be executable and named:

- `[target]_prepare.sh` - prepare the input state for your prover/verifier
- `[target]_prove.sh` - prove the input state
- `[target]_verify.sh` - verify the proof
- `[target]_measure.sh` - measure the proof and preprocessing sizes. By preprocessing we mean any circuit-specific state that a real application would need to persist between prover runs, e.g., proving key.

For `ligetron` with the `sha256` target, these are:

- `ligetron/sha256_prepare.sh`
- `ligetron/sha256_prove.sh`
- `ligetron/sha256_verify.sh`
- `ligetron/sha256_measure.sh`

The root `./benchmark.sh` will invoke them in a fixed way via `hyperfine` and our helper scripts. Your scripts should follow the APIs below.

#### API: `[target]_prepare.sh`

- Required environment variables:
  - `UTILS_BIN`: path to the `utils` binary in this repo (use it to generate inputs)
  - `INPUT_SIZE`: input size in bytes, if applicable
  - `STATE_JSON`: path to write a JSON file containing the input state for your prover/verifier
- Behavior:
  - Produce a single-line JSON (or pretty JSON) at `$STATE_JSON`. This JSON is opaque to the orchestrator; it is passed verbatim to your prover/verifier.
  - Exit non-zero on error.
- Example (Ligetron): builds a JSON containing the WASM program path, shader path, and args:

```10:35:ligetron/sha256_prepare.sh
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROGRAM_PATH="${SCRIPT_DIR}/ligero-prover/sdk/build/examples/sha256.wasm"
SHADER_PATH="${SCRIPT_DIR}/ligero-prover/shader"

GEN="$("$UTILS_BIN" sha256 -n "$INPUT_SIZE")"
MSG="$(printf "%s\n" "$GEN" | sed -n '1p')"
HEX_NO_PREFIX="$(printf "%s\n" "$GEN" | sed -n '2p')"

JQ_PROG='{program:$prog, "shader-path":$shader, packing:8192, "private-indices":[1], args:[{hex:$msg},{i64:$len},{hex:$dig}]}'

jq -nc \
  --arg prog "$PROGRAM_PATH" \
  --arg shader "$SHADER_PATH" \
  --arg msg "$MSG" \
  --arg dig "0x$HEX_NO_PREFIX" \
  --argjson len "$INPUT_SIZE" \
  "$JQ_PROG" > "$STATE_JSON"
```

#### API: `[target]_prove.sh`

- Required environment variables:
  - `STATE_JSON`: path to the JSON produced by prepare
- Behavior:
  - Run the prover for the state described by `$STATE_JSON`.
  - Should produce a proof artifact in a predictable location for size measurement (see measure API).
  - Exit non-zero on error.
- Example (Ligetron):

```7:11:ligetron/sha256_prove.sh
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
exec "$SCRIPT_DIR/ligero-prover/build/webgpu_prover" "$(cat "$STATE_JSON")"
```

#### API: `[target]_verify.sh`

- Required environment variables:
  - `STATE_JSON`: path to the JSON produced by prepare (and for CI, a proof will be generated beforehand)
- Behavior:
  - Run the verifier for the state described by `$STATE_JSON`.
  - Exit non-zero on error.
- Example (Ligetron):

```7:11:ligetron/sha256_verify.sh
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
exec "$SCRIPT_DIR/ligero-prover/build/webgpu_verifier" "$(cat "$STATE_JSON")"
```

#### API: `[target]_measure.sh`

- Required environment variables:
  - `STATE_JSON`: same JSON used for proving (you may need to run a quiet proof once to materialize the artifacts)
  - `SIZES_JSON`: output path for sizes JSON
- Behavior:
  - Output a JSON object containing `proof_size` and `preprocessing_size` (in bytes). Write it to `$SIZES_JSON`.
  - Exit non-zero on error.
- Example output:

```json
{ "proof_size": 475590, "preprocessing_size": 329524 }
```

- Example (Ligetron): finds `proof.data` and measures the WASM size as preprocessing:

```14:47:ligetron/sha256_measure.sh
"$SCRIPT_DIR/sha256_prove.sh" >/dev/null 2>&1 || true

proof_path="${PWD}/proof.data"
proof_size_bytes=$(stat -f %z "$proof_path" 2>/dev/null || stat -c %s "$proof_path")

WASM_PATH="${SCRIPT_DIR}/ligero-prover/sdk/build/examples/sha256.wasm"
PROVER_BIN_PATH="${SCRIPT_DIR}/ligero-prover/build/webgpu_prover"
wasm_size=$(stat -f %z "$WASM_PATH" 2>/dev/null || stat -c %s "$WASM_PATH")
preprocessing_size_bytes=$(( wasm_size ))

jq -n --argjson proof_size "$proof_size_bytes" --argjson preprocessing_size "$preprocessing_size_bytes" '{proof_size:$proof_size, preprocessing_size:$preprocessing_size}'
```

### 4) What the orchestrator and CI do for you

- The root `benchmark.sh` will, for each target and for each input size (driven by the `utils` crate):
  - Run `hyperfine` on your `[target]_prove.sh` and `[target]_verify.sh` to collect timing metrics.
  - Call our `measure_mem_avg.sh` to capture peak memory during proving.
  - Call your `[target]_measure.sh` to capture proof and preprocessing sizes.
  - Post-process `hyperfine` outputs into a `[target]_[size]_[system]_..._metrics.json` file.
- Ensure your `[target]_prove.sh` script performs a "lean" proof so memory is measured accurately.
- Ensure all four scripts are executable (`chmod +x`).

### 5) File naming recap for non-Rust systems

- Metrics: `[target]_[size]_[proving_system]_metrics.json`
- Memory report (created by our wrapper): `[target]_[size]_[proving_system]_mem_report.json`
- Sizes (produced by your `[target]_measure.sh`): contains `proof_size` and `preprocessing_size` as shown above

Use `ligetron` as a reference implementation.
