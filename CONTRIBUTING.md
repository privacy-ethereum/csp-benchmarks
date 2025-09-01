# How to Contribute

Howdy! Usual good software engineering practices apply. Write comments. Follow standard Rust coding practices where possible. Use `cargo fmt` and `cargo clippy` to tidy up formatting.

## What's Expected in the Contribution/PR

We depend on 3 JSON files for collecting benchmarks of the proving systems:

* Metrics
* Criterion report
* Memory report

When you add a new benchmark for a certain proving system, you should add a benchmarking code directory that are able to generate these JSON files. The directory should be included in the workspace(root-level `Cargo.toml`).

### Rules for JSON Files

#### Naming

* Metrics file: `[target]_[size]_[proving_system]_[(optional)feat]_metrics.json`
* Memory report: `[target]_[size]_[proving_system]_[(optional)feat]_mem_report.json`
* Criterion report: `[target]_[size]_[proving_system]_[(optional)feat]_prove` and `[target]_[size]_[proving_system]_[(optional)feat]_verify`  

NOTE: Criterion report files are generated under the `target/criterion` directory of workspace. Hence, you should apply above naming to criterion benchmark function IDs.

Example:

* When you want to add a benchmark for Binius with lookup for target function `sha256` with input size `2048`:
	+ Metrics: `sha256_2048_binius_with_lookup_metrics.json`
	+ Memory report: `sha256_2048_binius_with_lookup_mem_report.json`
	+ Criterion IDs: `sha256_2048_binius_with_lookup_prove` and `sha256_2048_binius_with_lookup_verify`

You can reference the existing benchmarks for naming. (e.g. [./binius/benches/sha256_bench.rs](https://github.com/privacy-scaling-explorations/csp-benchmarks/blob/collect-benchmarks-0/binius/benches/sha256_bench.rs))

#### Contents

* Metrics file: should be deserialized into the following struct:
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
  "name": "binius",
  "feat": "no_lookup",
  "is_zkvm": false,
  "target": "sha256",
  "input_size": 2048,
  "proof_duration": {
    "secs": 9,
    "nanos": 912673217
  },
  "verify_duration": {
    "secs": 3,
    "nanos": 933078432
  },
  "cycles": 0,
  "proof_size": 475590,
  "preprocessing_size": 329524,
  "peak_memory": 0
}
```
You can reference how to create JSON files of metrics in the existing benchmarks.(e.g. [./binius/benches/sha256_bench.rs](https://github.com/privacy-scaling-explorations/csp-benchmarks/blob/collect-benchmarks-0/binius/benches/sha256_bench.rs))

* Memory report: should be like the following:
```json
{
  "peak_memory": 44869222
}
```
We provide the shell script to make this JSON file - `./measure_mem_avg.sh`.  
You can reference how to use the script in README file of existing benchmarks.(e.g. [./binius/README.md](https://github.com/privacy-scaling-explorations/csp-benchmarks/blob/collect-benchmarks-0/binius/README.md#measure-sha256-ram-footprint))

For more details, please check the existing benchmarks like `binius` and `plonky2`.
