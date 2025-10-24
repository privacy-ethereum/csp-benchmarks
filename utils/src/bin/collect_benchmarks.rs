use glob::glob;
use serde_json::Value;
use std::path::{Path, PathBuf};
use std::time::Duration;
use std::{fs, io};
use utils::bench::Metrics;

/// Collect all JSON files in subdirectories of the workspace directory
/// containing benchmark metrics, and write them to a single JSON file
/// at `../collected_benchmarks.json`.
fn main() -> io::Result<()> {
    let mut benchmarks: Vec<Metrics> = Vec::new();
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
                        benchmarks.push(metrics);
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

    let output = serde_json::to_string_pretty(&benchmarks)?;
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
    let feat = &metrics.feat;

    if metrics.proof_duration.is_zero() {
        let crit_path_p = if feat.is_empty() {
            dir.parent().unwrap().join(format!(
                "target/criterion/{target}_{input_size}_{proving_system}/{target}_{input_size}_{proving_system}_prove/new/estimates.json"
            ))
        } else {
            dir.parent().unwrap().join(format!(
                "target/criterion/{target}_{input_size}_{proving_system}_{feat}/{target}_{input_size}_{proving_system}_{feat}_prove/new/estimates.json"
            ))
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
        let crit_path_v = if feat.is_empty() {
            dir.parent().unwrap().join(format!(
                "target/criterion/{target}_{input_size}_{proving_system}/{target}_{input_size}_{proving_system}_verify/new/estimates.json"
            ))
        } else {
            dir.parent().unwrap().join(format!(
                "target/criterion/{target}_{input_size}_{proving_system}_{feat}/{target}_{input_size}_{proving_system}_{feat}_verify/new/estimates.json"
            ))
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
        let mem_path = if feat.is_empty() {
            dir.join(format!(
                "{target}_{input_size}_{proving_system}_mem_report.json"
            ))
        } else {
            dir.join(format!(
                "{target}_{input_size}_{proving_system}_{feat}_mem_report.json"
            ))
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
