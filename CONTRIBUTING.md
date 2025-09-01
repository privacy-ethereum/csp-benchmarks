# How to contribute
Howdy! Usual good software engineering practices apply. Write comments. Follow standard Rust coding practices where possible. Use `cargo fmt` and `cargo clippy` to tidy up formatting.

## What's expected in the contribution/PR
We depend on 3 JSON files for collecting benchmark of the proving systems - metrics, criterion report and memory report.
When you add new benchmark for certain proving system, you should add directory which includes JSON files, into the repo.
The JSON files should follow the certain rules, in order to be collected by our collecting program.

### Rules for JSON files
1. Naming
The name of metrics file should be like following: 
`[target]_[size]_[proving_system]_[(optional)feat]_metrics.json`

The memory report should be named as similar:
`[target]_[size]_[proving_system]_[(optional)feat]_mem_report.json`

The naming of criterion report is similar, too. It's generated in the `target/criterion` dir of workspace, not in your dir.
Since the report JSON files are inside the dir with name of its bench ID, you should name the bench ID with above rule - `[target]_[size]_[proving_system]_[(optional)feat]_prove` and `[target]_[size]_[proving_system]_[(optional)feat]_verify`.

Example: When you want to add benchmark for binius with lookup for target function `sha256` with input size `2048`, 
```
metrics: sha256_2048_binius_with_lookup_metrics.json
mem report: sha256_2048_binius_with_lookup_mem_report.json
criterion IDs: sha256_2048_binius_with_lookup_prove, sha256_2048_binius_with_lookup_verify
```

2. Contents
The contents of metrics file should be deserialized into the following struct.
```rust
#[serde_as]
#[derive(Serialize, Deserialize, Tabled, Clone)]
pub struct Metrics {
    pub name: String,
    pub feat: String,
    pub target: String,
    #[tabled(display_with = "display_bytes")]
    pub input_size: usize,
    #[tabled(display_with = "display_bytes")]
    pub proof_size: usize,
    #[tabled(display_with = "display_bytes")]
    pub preprocessing_size: usize,
    #[tabled(display_with = "display_duration")]
    pub proof_duration: Duration,
    #[tabled(display_with = "display_duration")]
    pub verify_duration: Duration,
    #[tabled(display_with = "display_bytes")]
    pub peak_memory: usize,
}
```
Example: 
```json
{
  "name": "binius",
  "feat": "no_lookup",
  "target": "sha256",
  "input_size": 2048,
  "proof_size": 475590,
  "preprocessing_size": 329524,
  "proof_duration": {
    "secs": 0,
    "nanos": 0
  },
  "verify_duration": {
    "secs": 0,
    "nanos": 0
  },
  "peak_memory": 0
}
```

The contents of memory report should be like the following:

Example:
```json
{
  "runs": 10,
  "average_bytes": 44869222
}
```
Here, the `runs` means how many times we repeat the process, in order to get the average RAM usage.

For more details, please check the existing benchmarks like `binius`, and `plonky2`.