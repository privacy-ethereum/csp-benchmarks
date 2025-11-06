use clap::Parser;
use glob::glob;
use regex::Regex;
use serde::Deserialize;
use serde_json::Value;
use std::borrow::Cow;
use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::time::Duration;
use utils::bench::Metrics;
use utils::harness::{AuditStatus, BenchProperties};

#[derive(clap::Args, Debug, Clone, Default)]
struct BenchPropsArgs {
    #[arg(long)]
    proving_system: Option<String>,
    #[arg(long)]
    field_curve: Option<String>,
    #[arg(long)]
    iop: Option<String>,
    #[arg(long)]
    pcs: Option<String>,
    #[arg(long)]
    arithm: Option<String>,
    #[arg(long)]
    security_bits: Option<u64>,
    #[arg(long)]
    is_pq: Option<bool>,
    #[arg(long)]
    is_maintained: Option<bool>,
    #[arg(long)]
    is_zk: Option<bool>,
    #[arg(long)]
    is_audited: Option<String>,
    #[arg(long)]
    isa: Option<String>,
}

impl From<BenchPropsArgs> for BenchProperties {
    fn from(a: BenchPropsArgs) -> Self {
        BenchProperties {
            proving_system: a.proving_system.map(Cow::Owned),
            field_curve: a.field_curve.map(Cow::Owned),
            iop: a.iop.map(Cow::Owned),
            pcs: a.pcs.map(Cow::Owned),
            arithm: a.arithm.map(Cow::Owned),
            security_bits: a.security_bits,
            is_pq: a.is_pq,
            is_maintained: a.is_maintained,
            is_zk: a.is_zk,
            is_audited: a.is_audited.map(|s| AuditStatus::from_str(&s).unwrap()),
            isa: a.isa.map(Cow::Owned),
        }
    }
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

    /// Mark as zkVM system (default: false)
    #[arg(long, default_value_t = false)]
    is_zkvm: bool,

    /// Optional path to properties file (JSON) to populate BenchProperties
    #[arg(long)]
    properties: Option<PathBuf>,

    /// CLI overrides for BenchProperties (fields not provided remain unchanged)
    #[command(flatten)]
    props: BenchPropsArgs,
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

        let file_props = match &cli.properties {
            Some(p) => load_properties_json(p)?,
            None => BenchProperties::default(),
        };
        let override_props: BenchProperties = cli.props.clone().into();
        let bench_properties = merge_props(file_props, override_props);

        let mut metrics = Metrics::new(
            proving_system.clone(),
            feat,
            cli.is_zkvm,
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

fn load_properties_json(path: &Path) -> std::io::Result<BenchProperties> {
    let s = fs::read_to_string(path)?;
    serde_json::from_str::<BenchProperties>(&s).map_err(|e| io_err(&e.to_string()))
}

fn merge_props(mut base: BenchProperties, override_: BenchProperties) -> BenchProperties {
    macro_rules! take {
        ($f:ident) => {
            if override_.$f.is_some() {
                base.$f = override_.$f;
            }
        };
    }
    take!(proving_system);
    take!(field_curve);
    take!(iop);
    take!(pcs);
    take!(arithm);
    take!(security_bits);
    take!(is_pq);
    take!(is_maintained);
    take!(is_zk);
    take!(is_audited);
    take!(isa);
    base
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
