use human_repr::{HumanCount, HumanDuration};
use serde::Serialize;
use serde_with::{DurationNanoSeconds, serde_as};
use std::{
    fmt::Display,
    sync::{
        Arc,
        atomic::{AtomicBool, AtomicUsize, Ordering},
    },
    thread,
    time::Duration,
};
use tabled::{Table, Tabled, settings::Style};

fn get_current_memory_usage() -> Result<usize, std::io::Error> {
    unsafe {
        let mut out: libc::rusage = std::mem::zeroed();
        libc::getrusage(libc::RUSAGE_SELF, &mut out);
        #[cfg(target_os = "linux")]
        {
            Ok(out.ru_maxrss as usize * 1024)
        }
        #[cfg(target_os = "macos")]
        {
            Ok(out.ru_maxrss as usize)
        }
        #[cfg(not(any(target_os = "linux", target_os = "macos")))]
        {
            compile_error!("This crate only supports Linux and macOS for memory measurement");
        }
    }
}

pub fn measure_peak_memory<R, F: FnOnce() -> R>(func: F) -> (R, usize) {
    let peak = Arc::new(AtomicUsize::new(0));
    let stop = Arc::new(AtomicBool::new(false));

    let peak_clone = Arc::clone(&peak);
    let stop_clone = Arc::clone(&stop);
    let monitor = thread::spawn(move || {
        while !stop_clone.load(Ordering::Relaxed) {
            if let Ok(mem) = get_current_memory_usage() {
                peak_clone.fetch_max(mem, Ordering::Relaxed);
            }
            thread::sleep(Duration::from_millis(10));
        }
    });

    let result = func();

    stop.store(true, Ordering::Relaxed);
    monitor.join().unwrap();

    (result, peak.load(Ordering::Relaxed))
}

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

pub fn benchmark<T: Display + Clone, F>(func: F, inputs: &[T], file: &str)
where
    F: Fn(T) -> Metrics,
{
    let mut results = Vec::new();
    for input in inputs {
        let (mut metrics, peak_memory) = measure_peak_memory(|| func(input.clone()));
        metrics.peak_memory = peak_memory;
        results.push(metrics);
    }

    write_csv(file, &results);
}

pub fn write_csv(out_path: &str, results: &[Metrics]) {
    let mut out = csv::WriterBuilder::new().from_path(out_path).unwrap();

    let mut all_metrics = Vec::new();

    for metric in results {
        out.serialize(metric).expect("Could not serialize");
        out.flush().expect("Could not flush");
        all_metrics.push(metric);
    }

    out.flush().expect("Could not flush");

    let mut table = Table::new(&all_metrics);
    table.with(Style::modern());
    println!("{table}");
}

#[serde_as]
#[derive(Serialize, Tabled, Clone, Copy)]
pub struct SubMetrics {
    #[tabled(display_with = "display_bytes")]
    pub input_size: usize,
    #[tabled(display_with = "display_bytes")]
    pub proof_size: usize,
    #[tabled(display_with = "display_bytes")]
    pub proving_peak_memory: usize,
    #[tabled(display_with = "display_bytes")]
    pub preprocessing_size: usize,
    #[tabled(display_with = "display_bytes")]
    pub preprocessing_peak_memory: usize,
}

impl SubMetrics {
    pub fn new(size: usize) -> Self {
        SubMetrics {
            input_size: size,
            proof_size: 0,
            proving_peak_memory: 0,
            preprocessing_size: 0,
            preprocessing_peak_memory: 0,
        }
    }
}

pub fn display_submetrics(metrics: &[SubMetrics]) -> String {
    if metrics.is_empty() {
        return String::new();
    }
    let mut table = Table::new(metrics);
    table.with(Style::modern());
    table.to_string()
}

pub fn write_json_submetrics(output_path: &str, metrics: &SubMetrics) {
    let json = serde_json::to_string_pretty(metrics).unwrap();
    std::fs::write(output_path, json).unwrap();
}
