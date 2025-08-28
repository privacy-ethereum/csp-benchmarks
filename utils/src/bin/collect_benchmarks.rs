use serde_json::{Value, Map};
use utils::bench::CollectedMetrics;
use std::collections::HashMap;
use std::time::Duration;
use std::{fs, io};
use std::path::{Path, PathBuf};

fn main() -> io::Result<()> {
    let target = "sha256".to_string();
    let input_size = 2048;

    let mut prov_systems: HashMap<String, Vec<String>> = HashMap::new();
    prov_systems.insert("binius".to_string(), vec!["with_lookup".to_string(), "no_lookup".to_string()]);
    prov_systems.insert("plonky2".to_string(), vec!["no_lookup".to_string()]);
    prov_systems.insert("powdr".to_string(), vec![]);
    prov_systems.insert("provekit".to_string(), vec![]);

    let prov_sys_names = prov_systems.keys().collect::<Vec<_>>();

    let mut benchmarks: HashMap<String, CollectedMetrics> = HashMap::new();
    
    let root_dir = workspace_dir();
    for entry in fs::read_dir(root_dir)? {
        let path = entry?.path();
        if path.is_dir() {
            // println!("Processing {:?}", path);
            let path_str = path.file_name().unwrap().to_str().unwrap();
            for prov_sys in &prov_sys_names {
                if path_str.contains(*prov_sys){
                    let feats = prov_systems.get(*prov_sys).unwrap();
                    if feats.is_empty() {
                        let metrics = match extract_metrics(&path, &target, input_size, prov_sys, None) {
                            Ok(m) => m,
                            Err(err) => { eprintln!("Error in {:?}: {}", path, err); continue }
                        };
                        benchmarks.insert(path.file_name().unwrap().to_str().unwrap().to_owned(), metrics);
                    } else {
                        for feat in feats {
                            let metrics = match extract_metrics(&path, &target, input_size, prov_sys, Some(feat)) {
                                Ok(m) => m,
                                Err(err) => { eprintln!("Error in {:?}: {}", path, err); continue }
                            };
                            benchmarks.insert(path.file_name().unwrap().to_str().unwrap().to_owned(), metrics);
                        }
                    }
                }
            }
        }
    }

    let output = serde_json::to_string_pretty(&benchmarks).unwrap();
    std::fs::write("collected_benchmarks.json", output).unwrap();

    Ok(())
}

fn extract_metrics(dir: &Path, target: &String, input_size: usize, prov_sys: &String, feat: Option<&String>) -> io::Result<CollectedMetrics> {
    let crit_path_p = match feat {
        Some(feat) => dir.join(format!("../target/criterion/{target}_{input_size}_{prov_sys}_{feat}/{target}_{input_size}_{prov_sys}_{feat}_prove/new/estimates.json")),
        None => dir.join(format!("../target/criterion/{target}_{input_size}_{prov_sys}/{target}_{input_size}_{prov_sys}_prove/new/estimates.json")),
    };
    let crit_path_v = match feat {
        Some(feat) => dir.join(format!("../target/criterion/{target}_{input_size}_{prov_sys}_{feat}/{target}_{input_size}_{prov_sys}_{feat}_verify/new/estimates.json")),
        None => dir.join(format!("../target/criterion/{target}_{input_size}_{prov_sys}/{target}_{input_size}_{prov_sys}_verify/new/estimates.json")),
    };
    let mem_path = match feat {
        Some(feat)=> dir.join(format!("{target}_{input_size}_{prov_sys}_{feat}_mem_report.json")),
        None => dir.join(format!("{target}_{input_size}_{prov_sys}_mem_report.json")),
    };
    let sub_path = match feat {
        Some(feat) => dir.join(format!("{target}_{input_size}_{prov_sys}_{feat}_sub_report.json")),
        None => dir.join(format!("{target}_{input_size}_{prov_sys}_sub_report.json")),
    };

    let proof_crit: Value = serde_json::from_str(&fs::read_to_string(&crit_path_p)?)?;
    let verify_crit: Value = serde_json::from_str(&fs::read_to_string(&crit_path_v)?)?;
    let mem: Value = serde_json::from_str(&fs::read_to_string(&mem_path)?)?;
    let sub: Value = serde_json::from_str(&fs::read_to_string(&sub_path)?)?;

    let mut merged = Map::new();

    if let Some(est) = proof_crit.get("mean").and_then(|m| m.get("point_estimate")) {
        merged.insert("proof_time_estimate".to_owned(), est.clone());
    }
    if let Some(est) = verify_crit.get("mean").and_then(|m| m.get("point_estimate")) {
        merged.insert("verify_time_estimate".to_owned(), est.clone());
    }
    if let Some(avg) = mem.get("average_bytes") {
        merged.insert("average_memory_bytes".to_owned(), avg.clone());
    }

    if let Value::Object(map) = sub {
        for (k, v) in map {
            if k.ends_with("_memory") { continue; }
            merged.insert(k.clone(), v.clone());
        }
    }

    // let obj = Value::Object(merged);
    // fs::write(out_path, serde_json::to_string_pretty(&obj)?)?;

    let mut metrics = CollectedMetrics::new(
        dir.file_name().unwrap().to_str().unwrap().to_owned(),
        feat.map(|f| f.to_owned()).unwrap_or_default(),
        target.to_string(),
        input_size,
    );
    metrics.preprocessing_size = merged.get("preprocessing_size").unwrap().as_u64().unwrap() as usize;
    metrics.proof_duration = Duration::from_nanos(merged.get("proof_time_estimate").unwrap().as_f64().unwrap().round() as u64);
    metrics.verify_duration = Duration::from_nanos(merged.get("verify_time_estimate").unwrap().as_f64().unwrap().round() as u64);
    metrics.peak_memory = merged.get("average_memory_bytes").unwrap().as_u64().unwrap() as usize;

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
