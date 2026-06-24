pub const MOBILE_SAMPLE_COUNT: usize = 10;
pub const MOBILE_BREAK_SECS: u64 = 5;

pub fn format_prove_ms_summary(samples: &[u128]) -> String {
    assert!(!samples.is_empty());

    let mut sorted = samples.to_vec();
    sorted.sort_unstable();

    let count = samples.len();
    let mean = samples.iter().sum::<u128>() as f64 / count as f64;
    let median = if count % 2 == 0 {
        (sorted[count / 2 - 1] + sorted[count / 2]) as f64 / 2.0
    } else {
        sorted[count / 2] as f64
    };
    let variance = samples
        .iter()
        .map(|sample| {
            let delta = *sample as f64 - mean;
            delta * delta
        })
        .sum::<f64>()
        / count as f64;
    let stddev = variance.sqrt();
    let raw = samples
        .iter()
        .map(u128::to_string)
        .collect::<Vec<_>>()
        .join("|");

    format!(
        "prove_time_ms={mean:.3},prove_time_mean_ms={mean:.3},prove_time_median_ms={median:.3},prove_time_min_ms={},prove_time_max_ms={},prove_time_stddev_ms={stddev:.3},sample_count={count},samples_ms={raw}",
        sorted[0],
        sorted[count - 1],
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_expected_keys() {
        let summary = format_prove_ms_summary(&[1, 2, 3, 4]);
        assert!(summary.contains("prove_time_ms=2.500"));
        assert!(summary.contains("prove_time_median_ms=2.500"));
        assert!(summary.contains("sample_count=4"));
        assert!(summary.contains("samples_ms=1|2|3|4"));
    }
}
