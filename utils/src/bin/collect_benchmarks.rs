use chrono::Utc;
use glob::glob;
use serde::Serialize;
use serde_json::Value;
use serde_with::{DurationNanoSeconds, serde_as, skip_serializing_none};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::time::Duration;
use std::{env, fs, io};
use utils::bench::Metrics;
use utils::harness::BenchProperties;

/// Top-level output structure for collected benchmark results.
#[derive(Serialize)]
struct CollectedBenchmarks {
    metadata: Metadata,
    systems: BTreeMap<String, BenchProperties>,
    measurements: Vec<Measurement>,
}

/// Origin metadata for the collected benchmark run.
#[skip_serializing_none]
#[derive(Serialize)]
struct Metadata {
    timestamp: String,
    commit_sha: Option<String>,
    workflow_run_url: Option<String>,
    artifact_urls: Option<Vec<String>>,
}

/// A single benchmark measurement, referencing a system by key.
#[serde_as]
#[skip_serializing_none]
#[derive(Serialize)]
struct Measurement {
    system: String,
    target: String,
    input_size: usize,
    #[serde_as(as = "DurationNanoSeconds")]
    proof_duration: Duration,
    #[serde_as(as = "DurationNanoSeconds")]
    verify_duration: Duration,
    cycles: Option<u64>,
    proof_size: usize,
    preprocessing_size: usize,
    num_constraints: usize,
    peak_memory: usize,
}

/// Compute the unique system key from a metrics entry.
fn system_key(name: &str, feat: &Option<String>) -> String {
    match feat {
        Some(f) if !f.is_empty() => format!("{}_{}", name, f),
        _ => name.to_string(),
    }
}

/// Build [`Metadata`] from environment variables, if available.
fn build_metadata() -> Metadata {
    let timestamp = Utc::now().to_rfc3339();
    let commit_sha = env::var("COMMIT_SHA").ok().filter(|s| !s.is_empty());
    let workflow_run_url = env::var("WORKFLOW_RUN_URL").ok().filter(|s| !s.is_empty());
    let artifact_urls = env::var("ARTIFACT_URLS")
        .ok()
        .filter(|s| !s.is_empty())
        .map(|s| s.split(',').map(|u| u.trim().to_string()).collect());
    Metadata {
        timestamp,
        commit_sha,
        workflow_run_url,
        artifact_urls,
    }
}

/// Collect all JSON files in subdirectories of the workspace directory
/// containing benchmark metrics, and write them to a single JSON file
/// at `../collected_benchmarks.json`.
fn main() -> io::Result<()> {
    let mut all_metrics: Vec<Metrics> = Vec::new();
    let mut had_errors = false;
    let root_dir = workspace_dir();
    for entry in fs::read_dir(root_dir)? {
        let path = entry?.path();
        if path.is_dir() {
            let metrics_file_paths = find_metrics_files(&path);
            for metrics_file_path in metrics_file_paths {
                println!("Extracting metrics from {}", metrics_file_path.display());
                match extract_metrics(&path, &metrics_file_path) {
                    Ok((metrics, errors)) => {
                        all_metrics.push(metrics);
                        had_errors |= errors;
                    }
                    Err(e) => {
                        eprintln!(
                            "\n===== WARNING: failed to parse metrics file =====\n  file: {}\n  error: {}\n===============================================\n",
                            metrics_file_path.display(),
                            e
                        );
                        continue;
                    }
                }
            }
        }
    }

    let mut systems = BTreeMap::new();
    let mut measurements = Vec::new();
    for m in all_metrics {
        let key = system_key(&m.name, &m.feat);
        systems.entry(key.clone()).or_insert(m.bench_properties);
        measurements.push(Measurement {
            system: key,
            target: m.target,
            input_size: m.input_size,
            proof_duration: m.proof_duration,
            verify_duration: m.verify_duration,
            cycles: m.cycles,
            proof_size: m.proof_size,
            preprocessing_size: m.preprocessing_size,
            num_constraints: m.num_constraints,
            peak_memory: m.peak_memory,
        });
    }

    let collected = CollectedBenchmarks {
        metadata: build_metadata(),
        systems,
        measurements,
    };

    let output = serde_json::to_string_pretty(&collected)?;
    std::fs::write("../collected_benchmarks.json", output)?;

    if had_errors {
        Err(io::Error::other(
            "Metrics extraction had errors, see the logs for details",
        ))
    } else {
        Ok(())
    }
}

/// Extract `Metrics` from JSON file `metrics_file_path` and fill in any missing
/// fields by reading from Criterion's JSON files.
///
/// Specifically, this function looks for fields `proof_duration` and
/// `verify_duration` in the JSON file and fills them in with the mean
/// execution times reported by Criterion's JSON files, if they are not
/// already set. It also fills in the `peak_memory` field if it is not
/// already set, using the memory usage reported by the `mem_report` JSON
/// file.
///
/// Returns `Metrics` if successful.
fn extract_metrics(dir: &Path, metrics_file_path: &Path) -> io::Result<(Metrics, bool)> {
    let mut had_errors = false;
    let metrics_json: Value = serde_json::from_str(&fs::read_to_string(metrics_file_path)?)?;

    let mut metrics: Metrics = serde_json::from_value(metrics_json)?;

    let target = &metrics.target;
    let input_size = metrics.input_size;
    let proving_system = &metrics.name;
    let feat = metrics.feat.as_deref();

    if metrics.proof_duration.is_zero() {
        let crit_path_p = match feat {
            Some(f) if !f.is_empty() => dir.parent().unwrap().join(format!(
                "target/criterion/{target}_{input_size}_{proving_system}_{f}/{target}_{input_size}_{proving_system}_{f}_prove/new/estimates.json"
            )),
            _ => dir.parent().unwrap().join(format!(
                "target/criterion/{target}_{input_size}_{proving_system}/{target}_{input_size}_{proving_system}_prove/new/estimates.json"
            )),
        };
        if crit_path_p.exists() {
            println!("Reading proof duration from {}", crit_path_p.display());
            match fs::read_to_string(&crit_path_p) {
                Ok(contents) => match serde_json::from_str::<Value>(&contents) {
                    Ok(proof_crit) => {
                        if let Some(est) =
                            proof_crit.get("mean").and_then(|m| m.get("point_estimate"))
                            && let Some(f) = est.as_f64()
                        {
                            metrics.proof_duration = Duration::from_nanos(f.round() as u64);
                        }
                    }
                    Err(e) => {
                        eprintln!(
                            "\n===== WARNING: failed to parse proof estimates =====\n  file: {}\n  error: {}\n===================================================\n",
                            crit_path_p.display(),
                            e
                        );
                        had_errors = true;
                    }
                },
                Err(e) => {
                    eprintln!(
                        "\n===== WARNING: failed to read proof estimates =====\n  file: {}\n  error: {}\n==================================================\n",
                        crit_path_p.display(),
                        e
                    );
                    had_errors = true;
                }
            }
        } else {
            eprintln!(
                "\n===== WARNING: proof estimates.json not found =====\n  file: {}\n==================================================\n",
                crit_path_p.display()
            );
            had_errors = true;
        }
    }

    if metrics.verify_duration.is_zero() {
        let crit_path_v = match feat {
            Some(f) if !f.is_empty() => dir.parent().unwrap().join(format!(
                "target/criterion/{target}_{input_size}_{proving_system}_{f}/{target}_{input_size}_{proving_system}_{f}_verify/new/estimates.json"
            )),
            _ => dir.parent().unwrap().join(format!(
                "target/criterion/{target}_{input_size}_{proving_system}/{target}_{input_size}_{proving_system}_verify/new/estimates.json"
            )),
        };
        if crit_path_v.exists() {
            println!("Reading verify duration from {}", crit_path_v.display());
            match fs::read_to_string(&crit_path_v) {
                Ok(contents) => match serde_json::from_str::<Value>(&contents) {
                    Ok(verify_crit) => {
                        if let Some(est) = verify_crit
                            .get("mean")
                            .and_then(|m| m.get("point_estimate"))
                            && let Some(f) = est.as_f64()
                        {
                            metrics.verify_duration = Duration::from_nanos(f.round() as u64);
                        }
                    }
                    Err(e) => {
                        eprintln!(
                            "\n===== WARNING: failed to parse verify estimates =====\n  file: {}\n  error: {}\n====================================================\n",
                            crit_path_v.display(),
                            e
                        );
                        had_errors = true;
                    }
                },
                Err(e) => {
                    eprintln!(
                        "\n===== WARNING: failed to read verify estimates =====\n  file: {}\n  error: {}\n===================================================\n",
                        crit_path_v.display(),
                        e
                    );
                    had_errors = true;
                }
            }
        } else {
            eprintln!(
                "\n===== WARNING: verify estimates.json not found =====\n  file: {}\n===================================================\n",
                crit_path_v.display()
            );
            had_errors = true;
        }
    }

    if metrics.peak_memory == 0 {
        let mem_path = match feat {
            Some(f) if !f.is_empty() => dir.join(format!(
                "{target}_{input_size}_{proving_system}_{f}_mem_report.json"
            )),
            _ => dir.join(format!(
                "{target}_{input_size}_{proving_system}_mem_report.json"
            )),
        };
        if mem_path.exists() {
            println!("Reading peak memory from {}", mem_path.display());
            match fs::read_to_string(&mem_path) {
                Ok(contents) => match serde_json::from_str::<Value>(&contents) {
                    Ok(mem) => {
                        if let Some(m) = mem.get("peak_memory") {
                            metrics.peak_memory = m.as_u64().unwrap_or(0) as usize;
                        }
                    }
                    Err(e) => {
                        eprintln!(
                            "\n===== WARNING: failed to parse memory report =====\n  file: {}\n  error: {}\n==================================================\n",
                            mem_path.display(),
                            e
                        );
                        had_errors = true;
                    }
                },
                Err(e) => {
                    eprintln!(
                        "\n===== WARNING: failed to read memory report =====\n  file: {}\n  error: {}\n=================================================\n",
                        mem_path.display(),
                        e
                    );
                    had_errors = true;
                }
            }
        } else {
            eprintln!(
                "\n===== WARNING: memory report not found =====\n  file: {}\n===========================================\n",
                mem_path.display()
            );
            had_errors = true;
        }
    }

    Ok((metrics, had_errors))
}

/// Returns the root directory of the current workspace, as determined by the
/// `cargo locate-project` command.
fn workspace_dir() -> PathBuf {
    let output = std::process::Command::new(env!("CARGO"))
        .arg("locate-project")
        .arg("--workspace")
        .arg("--message-format=plain")
        .output()
        .unwrap()
        .stdout;
    let cargo_path = Path::new(std::str::from_utf8(&output).unwrap().trim());
    cargo_path.parent().unwrap().to_path_buf()
}

/// Try to find a file(s) matching "*_metrics.json" in `dir`.
/// Returns `Vec<PathBuf>`.
fn find_metrics_files(dir: &Path) -> Vec<PathBuf> {
    // Construct the pattern like "dir/*_metrics.json"
    let pattern = dir.join("*_metrics.json").to_string_lossy().into_owned();

    let mut metrics_files: Vec<PathBuf> = Vec::new();

    // Iterate over matching entries
    for entry in glob(&pattern).unwrap() {
        match entry {
            Ok(path) => {
                metrics_files.push(path);
            }
            Err(e) => eprintln!("Glob error: {}", e),
        }
    }

    metrics_files
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::borrow::Cow;
    use utils::harness::AuditStatus;

    #[test]
    fn test_system_key_no_feat() {
        assert_eq!(system_key("binius64", &None), "binius64");
    }

    #[test]
    fn test_system_key_empty_feat() {
        assert_eq!(system_key("binius64", &Some(String::new())), "binius64");
    }

    #[test]
    fn test_system_key_with_feat() {
        assert_eq!(
            system_key("circom", &Some("groth16".to_string())),
            "circom_groth16"
        );
    }

    #[test]
    fn test_collected_benchmarks_structure() {
        let props = BenchProperties {
            proving_system: Cow::Owned("Binius64".into()),
            field_curve: Cow::Owned("GHASH binary field".into()),
            iop: Cow::Owned("Binius64".into()),
            pcs: Some(Cow::Owned("Binius64".into())),
            arithm: Cow::Owned("Binius64".into()),
            is_zk: false,
            is_zkvm: false,
            security_bits: 96,
            is_pq: true,
            is_maintained: true,
            is_audited: AuditStatus::NotAudited,
            isa: None,
        };

        let mut systems = BTreeMap::new();
        systems.insert("binius64".to_string(), props);

        let measurements = vec![Measurement {
            system: "binius64".to_string(),
            target: "sha256".to_string(),
            input_size: 128,
            proof_duration: Duration::from_nanos(12345000),
            verify_duration: Duration::from_nanos(6789000),
            cycles: None,
            proof_size: 1024,
            preprocessing_size: 2048,
            num_constraints: 5000,
            peak_memory: 100000,
        }];

        let collected = CollectedBenchmarks {
            metadata: Metadata {
                timestamp: "2026-01-01T00:00:00+00:00".to_string(),
                commit_sha: None,
                workflow_run_url: None,
                artifact_urls: None,
            },
            systems,
            measurements,
        };

        let json_str = serde_json::to_string_pretty(&collected).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json_str).unwrap();

        // Verify top-level structure
        assert!(parsed.get("metadata").is_some());
        assert!(parsed.get("systems").is_some());
        assert!(parsed.get("measurements").is_some());

        // Verify systems is a map with the system key
        let systems = parsed["systems"].as_object().unwrap();
        assert!(systems.contains_key("binius64"));
        assert_eq!(systems["binius64"]["proving_system"], "Binius64");

        // Verify measurements is an array referencing the system
        let measurements = parsed["measurements"].as_array().unwrap();
        assert_eq!(measurements.len(), 1);
        assert_eq!(measurements[0]["system"], "binius64");
        assert_eq!(measurements[0]["target"], "sha256");
        assert_eq!(measurements[0]["input_size"], 128);
        assert_eq!(measurements[0]["proof_duration"], 12345000);
        assert_eq!(measurements[0]["verify_duration"], 6789000);

        // Verify system properties are NOT in measurements
        assert!(measurements[0].get("proving_system").is_none());
        assert!(measurements[0].get("field_curve").is_none());
        assert!(measurements[0].get("iop").is_none());

        // Verify cycles is not serialized when None
        assert!(measurements[0].get("cycles").is_none());
    }

    #[test]
    fn test_metadata_from_env() {
        // Without env vars set, metadata fields should be None
        let metadata = build_metadata();
        // Cannot guarantee env vars are unset, but the function should not panic
        let json = serde_json::to_string(&metadata).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        // Timestamp should always be present
        assert!(parsed.get("timestamp").is_some());
        assert!(!metadata.timestamp.is_empty());
        // When both are None, they should not appear in JSON (skip_serializing_none)
        if metadata.workflow_run_url.is_none() {
            assert!(parsed.get("workflow_run_url").is_none());
        }
    }
}
