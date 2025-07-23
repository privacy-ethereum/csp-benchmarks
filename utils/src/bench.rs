use human_repr::{HumanCount, HumanDuration};
use serde::Serialize;
use serde_with::{serde_as, DurationNanoSeconds};
use std::time::Duration;
use tabled::{settings::Style, Table, Tabled};

#[serde_as]
#[derive(Serialize, Tabled)]
pub struct Metrics {
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
    pub peak_memory: usize,
}

fn display_bytes(bytes: &usize) -> String {
    bytes.human_count_bytes().to_string()
}

fn display_cycles(cycles: &u64) -> String {
    cycles.human_count_bare().to_string()
}

fn display_duration(duration: &Duration) -> String {
    duration.human_duration().to_string()
}

impl Metrics {
    pub fn new(size: usize) -> Self {
        Metrics {
            input_size: size,
            proof_duration: Duration::default(),
            verify_duration: Duration::default(),
            cycles: 0,
            proof_size: 0,
            peak_memory: 0,
        }
    }
}

pub fn write_csv(out_path: &str, results: &[Metrics]) {
    let mut out = csv::WriterBuilder::new().from_path(out_path).unwrap();

    let mut all_metrics = Vec::new();

    for metric in results {
        out.serialize(&metric).expect("Could not serialize");
        out.flush().expect("Could not flush");
        all_metrics.push(metric);
    }

    out.flush().expect("Could not flush");

    let mut table = Table::new(&all_metrics);
    table.with(Style::modern());
    println!("{table}");
}
