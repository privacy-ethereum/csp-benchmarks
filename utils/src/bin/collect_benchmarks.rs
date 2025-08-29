use serde_json::Value;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::Duration;
use std::{fs, io};
use utils::bench::Metrics1;

fn main() -> io::Result<()> {
    let benchmark_target = "sha256".to_string();
    let benchmark_input_size = 2048;

    let mut proving_systems: HashMap<String, Vec<String>> = HashMap::new();
    proving_systems.insert(
        "binius".to_string(),
        vec!["with_lookup".to_string(), "no_lookup".to_string()],
    );
    proving_systems.insert("plonky2".to_string(), vec!["no_lookup".to_string()]);
    proving_systems.insert("powdr".to_string(), vec![]);
    proving_systems.insert("provekit".to_string(), vec![]);

    let proving_system_names = proving_systems.keys().collect::<Vec<_>>();

    let mut benchmarks: HashMap<String, Metrics1> = HashMap::new();

    let root_dir = workspace_dir();
    for entry in fs::read_dir(root_dir)? {
        let path = entry?.path();
        if path.is_dir() {
            let path_str = path.file_name().unwrap().to_str().unwrap();
            for proving_system in &proving_system_names {
                if path_str.contains(*proving_system) {
                    let features = proving_systems.get(*proving_system).unwrap();
                    if features.is_empty() {
                        let metrics = extract_metrics(
                            &path,
                            &benchmark_target,
                            benchmark_input_size,
                            proving_system,
                            None,
                        )?;
                        benchmarks.insert(path_str.to_owned(), metrics);
                    } else {
                        for feature in features {
                            let metrics = extract_metrics(
                                &path,
                                &benchmark_target,
                                benchmark_input_size,
                                proving_system,
                                Some(feature),
                            )?;
                            benchmarks.insert(format!("{path_str}_{feature}"), metrics);
                        }
                    }
                }
            }
        }
    }

    let output = serde_json::to_string_pretty(&benchmarks)?;
    std::fs::write("../collected_benchmarks.json", output)?;

    Ok(())
}

fn extract_metrics(
    dir: &Path,
    target: &String,
    input_size: usize,
    proving_system: &String,
    feature: Option<&String>,
) -> io::Result<Metrics1> {
    let crit_path_p = match feature {
        Some(feat) => dir
            .parent()
            .unwrap()
            .join(format!(
                "target/criterion/{target}_{input_size}_{proving_system}_{feat}/{target}_{input_size}_{proving_system}_{feat}_prove/new/estimates.json"
            )),
        None => dir
            .parent()
            .unwrap()
            .join(format!(
                "target/criterion/{target}_{input_size}_{proving_system}/{target}_{input_size}_{proving_system}_prove/new/estimates.json"
            )),
    };
    let crit_path_v = match feature {
        Some(feat) => dir
            .parent()
            .unwrap()
            .join(format!(
                "target/criterion/{target}_{input_size}_{proving_system}_{feat}/{target}_{input_size}_{proving_system}_{feat}_verify/new/estimates.json"
            )),
        None => dir
            .parent()
            .unwrap()
            .join(format!(
                "target/criterion/{target}_{input_size}_{proving_system}/{target}_{input_size}_{proving_system}_verify/new/estimates.json"
            )),
    };
    let mem_path = match feature {
        Some(feat) => dir.join(format!(
            "{target}_{input_size}_{proving_system}_{feat}_mem_report.json"
        )),
        None => dir.join(format!(
            "{target}_{input_size}_{proving_system}_mem_report.json"
        )),
    };
    let metrics_path = match feature {
        Some(feat) => dir.join(format!(
            "{target}_{input_size}_{proving_system}_{feat}_metrics.json"
        )),
        None => dir.join(format!(
            "{target}_{input_size}_{proving_system}_metrics.json"
        )),
    };

    let proof_crit: Value = serde_json::from_str(&fs::read_to_string(&crit_path_p)?)?;
    let verify_crit: Value = serde_json::from_str(&fs::read_to_string(&crit_path_v)?)?;
    let mem: Value = serde_json::from_str(&fs::read_to_string(&mem_path)?)?;
    let metrics: Value = serde_json::from_str(&fs::read_to_string(&metrics_path)?)?;

    let mut metrics : Metrics1 = serde_json::from_value(metrics.clone())?;
    if let Some(est) = proof_crit.get("mean").and_then(|m| m.get("point_estimate")) {
        metrics.proof_duration = Duration::from_nanos(est.as_f64().unwrap().round() as u64);
    }
    if let Some(est) = verify_crit
        .get("mean")
        .and_then(|m| m.get("point_estimate"))
    {
        metrics.verify_duration = Duration::from_nanos(est.as_f64().unwrap().round() as u64);
    }
    if let Some(avg) = mem.get("average_bytes") {
        metrics.peak_memory = avg.as_u64().unwrap() as usize;
    }

    Ok(metrics)
}

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
