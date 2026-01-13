use clap::Parser;
use glob::glob;
use regex::Regex;
use serde::Deserialize;
use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Duration;
use utils::bench::Metrics;
use utils::harness::BenchProperties;

#[derive(clap::Args, Debug, Clone, Default)]
struct BenchPropsArgs {
    #[arg(long)]
    proving_system: String,
    #[arg(long)]
    field_curve: String,
    #[arg(long)]
    iop: String,
    #[arg(long)]
    pcs: Option<String>,
    #[arg(long)]
    arithm: String,
    #[arg(long)]
    security_bits: u64,
    #[arg(long)]
    is_pq: bool,
    #[arg(long)]
    is_maintained: bool,
    #[arg(long)]
    is_zk: bool,
    #[arg(long)]
    is_audited: String,
    #[arg(long)]
    isa: Option<String>,
}

#[derive(Parser, Debug)]
#[command(author, version, about = "Format hyperfine + RAM outputs into Metrics JSON and clean up", long_about = None)]
struct Cli {
    /// Path to the non-Rust system directory (e.g., ./ligetron)
    #[arg(long, value_name = "DIR", default_value = ".")]
    system_dir: String,

    /// Override proving system name (defaults to basename of system_dir)
    #[arg(long)]
    name: Option<String>,

    /// Optional feature name (default empty)
    #[arg(long, default_value = "")]
    feature: Option<String>,

    /// Optional path to properties file (JSON) to populate BenchProperties
    #[arg(long)]
    properties: Option<PathBuf>,

    /// Optional path to number of constraints file (JSON)
    #[arg(long)]
    num_constraints_file: Option<PathBuf>,
}

#[derive(Deserialize)]
struct HyperfineRecord {
    mean: f64,
}

/// Formats hyperfine + RAM outputs into Metrics JSON and cleans up
fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    let system_dir = PathBuf::from(&cli.system_dir);
    if !system_dir.is_dir() {
        eprintln!("system_dir is not a directory: {}", system_dir.display());
        std::process::exit(2);
    }

    let proving_system = cli.name.unwrap_or_else(|| {
        system_dir
            .file_name()
            .unwrap()
            .to_string_lossy()
            .to_string()
    });

    // hyperfine files have the form: hyperfine_<target>_<size>_prover_metrics.json
    let pattern = system_dir.join("hyperfine_*_*_prover_metrics.json");
    let pattern = pattern.to_string_lossy().into_owned();
    let re =
        Regex::new(r"hyperfine_(?P<target>[^_]+)_(?P<size>[^_]+)_prover_metrics\.json$").unwrap();

    for entry in glob(&pattern).unwrap() {
        let prover_path = match entry {
            Ok(p) => p,
            Err(err) => {
                eprintln!("Glob error: {err}");
                continue;
            }
        };

        let file_name = prover_path.file_name().unwrap().to_string_lossy();
        let caps = match re.captures(&file_name) {
            Some(c) => c,
            None => continue,
        };
        let target = caps.name("target").unwrap().as_str().to_string();
        let size_str = caps.name("size").unwrap().as_str();
        let input_size: usize = size_str.parse().unwrap_or_else(|_| {
            eprintln!("Could not parse input size from {file_name}");
            std::process::exit(2);
        });

        let verifier_path = system_dir.join(format!(
            "hyperfine_{target}_{input_size}_verifier_metrics.json"
        ));
        let mem_path = system_dir.join(format!("{target}_{input_size}_mem_report.json"));
        let sizes_path = system_dir.join(format!("{target}_{input_size}_sizes.json"));

        // Parse hyperfine JSONs to extract mean seconds
        let prover_mean_sec = read_hyperfine_mean_seconds(&prover_path)?;
        println!("Reading prover time from {}", prover_path.display());
        let verifier_mean_sec = read_hyperfine_mean_seconds(&verifier_path)?;
        println!("Reading verifier time from {}", verifier_path.display());

        let feat = match cli.feature.as_deref() {
            Some(f) if !f.is_empty() => Some(f.to_string()),
            _ => None,
        };

        let bench_properties = match &cli.properties {
            Some(p) => load_properties_json(p)?,
            None => {
                eprintln!("No properties file provided");
                std::process::exit(2);
            }
        };

        let mut metrics = Metrics::new(
            proving_system.clone(),
            feat,
            target.clone(),
            input_size,
            bench_properties,
        );
        metrics.proof_duration = to_duration_ns(prover_mean_sec);
        metrics.verify_duration = to_duration_ns(verifier_mean_sec);

        if mem_path.exists()
            && let Ok(mem_bytes) = read_peak_memory_bytes(&mem_path)
        {
            println!("Reading peak memory from {}", mem_path.display());
            metrics.peak_memory = mem_bytes;
        }

        if sizes_path.exists()
            && let Ok((proof_size, preprocessing_size)) = read_sizes_bytes(&sizes_path)
        {
            println!("Reading sizes from {}", sizes_path.display());
            metrics.proof_size = proof_size;
            metrics.preprocessing_size = preprocessing_size;
        }

        if let Some(num_constraints_file) = &cli.num_constraints_file {
            if let Ok(num_constraints) =
                read_num_constraints_json(num_constraints_file, &target, input_size)
            {
                println!(
                    "Reading number of constraints from {}",
                    num_constraints_file.display()
                );
                metrics.num_constraints = num_constraints;
            } else {
                eprintln!(
                    "Could not read number of constraints from {}",
                    num_constraints_file.display()
                );
                std::process::exit(2);
            }
        }

        let out_file = system_dir.join(format!(
            "{target}_{input_size}_{proving_system}_metrics.json"
        ));
        utils::bench::write_json_metrics_file(out_file.to_str().unwrap(), &metrics);

        // Cleanup originals
        let _ = fs::remove_file(&prover_path);
        let _ = fs::remove_file(&verifier_path);
        let _ = fs::remove_file(&mem_path);
        let _ = fs::remove_file(&sizes_path);
    }

    Ok(())
}

/// Reads the number of constraints from the circuit_sizes.json file
/// for the given target and size.
/// The file contents are expected to be in the following format:
/// {
///   "target": {
///     "size": number_of_constraints
///   }
/// }
///
/// Returns the number of constraints for the given target and size.
fn read_num_constraints_json(
    num_constraints_file: &PathBuf,
    target: &str,
    size: usize,
) -> std::io::Result<usize> {
    let v: Value = serde_json::from_str(&fs::read_to_string(num_constraints_file)?)?;

    let target = v
        .get(target)
        .ok_or_else(|| io_err(&format!("missing {target} benchmark target")))?;
    let size = target.get(size.to_string()).ok_or_else(|| {
        io_err(&format!(
            "missing circuit size for the input of {size} bytes for {target} benchmark target"
        ))
    })?;
    Ok(size
        .as_u64()
        .ok_or_else(|| {
            io_err(&format!(
                "number of constraints value is not a number for the input of {size} bytes for {target} benchmark target"
            ))
        })? as usize)
}

fn load_properties_json(path: &Path) -> std::io::Result<BenchProperties> {
    let s = fs::read_to_string(path)?;
    serde_json::from_str::<BenchProperties>(&s).map_err(|e| io_err(&e.to_string()))
}

fn read_hyperfine_mean_seconds(path: &Path) -> std::io::Result<f64> {
    let v: Value = serde_json::from_str(&fs::read_to_string(path)?)?;
    let results = v
        .get("results")
        .and_then(|r| r.as_array())
        .ok_or_else(|| io_err("missing results array"))?;
    let first = results.first().ok_or_else(|| io_err("results empty"))?;
    let rec: HyperfineRecord =
        serde_json::from_value(first.clone()).map_err(|_| io_err("invalid hyperfine record"))?;
    Ok(rec.mean)
}

fn read_peak_memory_bytes(path: &Path) -> std::io::Result<usize> {
    let v: Value = serde_json::from_str(&fs::read_to_string(path)?)?;
    v.get("peak_memory")
        .and_then(|m| m.as_u64())
        .map(|n| n as usize)
        .ok_or_else(|| io_err("missing peak_memory"))
}

fn read_sizes_bytes(path: &Path) -> std::io::Result<(usize, usize)> {
    let v: Value = serde_json::from_str(&fs::read_to_string(path)?)?;
    let proof = v
        .get("proof_size")
        .and_then(|m| m.as_u64())
        .ok_or_else(|| io_err("missing proof_size"))? as usize;
    let prep = v
        .get("preprocessing_size")
        .and_then(|m| m.as_u64())
        .unwrap_or(0) as usize;
    Ok((proof, prep))
}

fn to_duration_ns(seconds: f64) -> Duration {
    let nanos = (seconds * 1_000_000_000.0).round() as u64;
    Duration::from_nanos(nanos)
}

fn io_err(msg: &str) -> std::io::Error {
    std::io::Error::other(msg)
}
