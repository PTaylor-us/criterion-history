use std::fs;
use std::path::Path;

mod plotter;

fn main() {
    let criterion_output = "target/criterion";

    let mut log = Log::new();

    read_dir_recursively(&mut log, criterion_output);

    plotter::plot(&log).unwrap();
}

#[derive(Debug)]
struct LogData {
    timestamp: u64,
    average_duration: f64,
}

type Log = std::collections::HashMap<String, Vec<LogData>>;

fn read_dir_recursively<P: AsRef<Path>>(log: &mut Log, path: P) {
    for entry in fs::read_dir(path).unwrap() {
        let dir = entry.unwrap();
        if dir.file_type().unwrap().is_dir() {
            if dir.file_name().into_string().unwrap().starts_with("BM_") {
                let file_name = dir.file_name().into_string().unwrap();
                let timestamp: u64 = file_name.trim_start_matches("BM_").parse().unwrap();

                let mut new_results = csv::Reader::from_path(dir.path().join("raw.csv")).unwrap();

                let first_record = new_results.records().next().unwrap().unwrap();
                let test_name = String::from(first_record.get(0).unwrap());
                let test_name = test_name + "::";
                let test_name = test_name + first_record.get(1).unwrap();
                let test_name = test_name + first_record.get(2).unwrap();

                let mut durations = Vec::new();
                for record in new_results.records() {
                    let record = record.unwrap();
                    let duration: f64 = record.get(5).unwrap().parse().unwrap();
                    let scaling_factor: f64 = match record.get(6) {
                        Some("us") => (1_f64 / 1_000_000_f64),
                        Some("ns") => (1_f64 / 1_000_000_000_f64),
                        Some("ps") => (1_f64 / 1_000_000_000_000_f64),
                        other => panic!("unit not supported: {}", other.unwrap()),
                    };
                    let iterations: f64 = record.get(7).unwrap().parse().unwrap();
                    let duration: f64 = duration * scaling_factor / iterations;
                    durations.push(duration);
                }
                let average_duration: f64 = durations.iter().sum::<f64>() / durations.len() as f64;

                let data = LogData {
                    timestamp,
                    average_duration,
                };
                let test_data = log.entry(test_name).or_insert_with(Vec::new);
                test_data.push(data);
            } else {
                read_dir_recursively(log, dir.path());
            }
        }
    }
}
