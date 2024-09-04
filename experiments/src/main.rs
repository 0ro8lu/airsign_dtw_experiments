mod plot;
mod test_results;
mod utils;

use plot::*;
use test_results::TestResult;
use utils::*;

use rust_dtw_algorithm::dynamic_time_warping_multivariate;
use std::env;
use std::fs::{read_dir, File};
use std::io::{self, BufRead};
use std::path::PathBuf;
use std::str;

const NUM_SIGNATURES: usize = 20;
const NUM_USERS: u32 = 5;
const MAX_THRESHOLD: usize = 500;
const THRESHOLD_STEP: usize = 20;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Check if the correct number of arguments is given
    if args.len() != 4 {
        eprintln!("Usage: program <directory_path> <min/max/avg>",);
        std::process::exit(1);
    }

    let dir_path = &args[1];
    let selector = &args[2];
    
    let elements_max_function: &mut dyn FnMut(&[f32]) -> f32 = &mut elements_max;
    let elements_min_function: &mut dyn FnMut(&[f32]) -> f32 = &mut elements_min;
    let elements_avg_function: &mut dyn FnMut(&[f32]) -> f32 = &mut elements_avg;

    let mut selector_function: &mut dyn FnMut(&[f32]) -> f32 = match selector.as_str() {
        "max" => elements_max_function,
        "min" => elements_min_function,
        "avg" => elements_avg_function,
        _ => panic!("{} is not a supported argument", selector),
    };

    let mut far_points: Vec<(f32, f32)> = Vec::new();
    let mut frr_points: Vec<(f32, f32)> = Vec::new();

    let mut index = 0;
    for threshold in (0..=MAX_THRESHOLD).rev().step_by(THRESHOLD_STEP) {
        let (step, mut results_array) = match &(*args[3]) {
            "full" => (20, vec![TestResult::new(); 1]),
            "reduced" => (5, vec![TestResult::new(); 4]),
            _ => panic!("{} is not a supported argument, either \"full\" or \"reduced\" are supported!", args[3]),
        };

        for test_entry_res in read_dir(dir_path).expect("Failed to read directory") {
            let test_entry = test_entry_res.expect("Failed to process test_entry_res directory");
            let test_file_name = test_entry.file_name().to_str().unwrap().to_owned();

            if test_file_name.starts_with(".") || !test_entry.file_type().unwrap().is_dir() {
                continue;
            }

            for signature_index in 20..=24 {
                let signature_file =
                    format!("{}/signature_{}.csv", test_file_name, signature_index);
                let testing_path = PathBuf::from(dir_path).join(&signature_file);

                let mut results_index = 0;
                for index in (0..20).step_by(step) {
                    dtw_on_signature(
                        dir_path,
                        &testing_path.to_string_lossy(),
                        threshold as f32,
                        &test_file_name,
                        index,
                        index + step,
                        &mut results_array[results_index],
                        &mut selector_function,
                    );

                    results_index += 1;
                }
            }
        }

        // average the results
        let results = average_result(&mut results_array);

        let rev_threshold = (MAX_THRESHOLD - (MAX_THRESHOLD - index * THRESHOLD_STEP)) as f32;
        let far_tuple = (
            rev_threshold,
            (results.false_accept / (NUM_USERS - 1)) as f32,
        );
        let frr_tuple = (rev_threshold, results.false_reject as f32);

        far_points.push(far_tuple);
        frr_points.push(frr_tuple);

        index += 1;
    }

    normalize_tuples(&mut far_points);
    normalize_tuples(&mut frr_points);

    plot_far_frr(far_points.clone(), frr_points.clone()).unwrap();
    plot_roc(frr_points.into_iter().rev().collect(), far_points).unwrap();
}

fn average_result(results_array: &mut[TestResult]) -> TestResult{
    let mut results = TestResult::new();
    
    for result in &mut *results_array {
        results.genuine_accept += result.genuine_accept;
        results.genuine_reject += result.genuine_reject;
        results.false_accept += result.false_accept;
        results.false_reject += result.false_reject;
    }

    results.genuine_accept = results.genuine_accept / results_array.len() as u32;
    results.genuine_reject = results.genuine_reject / results_array.len() as u32;
    results.false_accept = results.false_accept / results_array.len() as u32;
    results.false_reject = results.false_reject / results_array.len() as u32;

    results
}

fn dtw_on_signature(
    training_string: &str,
    testing_string: &str,
    threshold: f32,
    testing_user: &str,
    offset_index: usize,
    index_window: usize,
    results: &mut TestResult,
    selector_function: &mut dyn FnMut(&[f32]) -> f32,
) {
    assert!(
        offset_index < NUM_SIGNATURES,
        "offset_index is bigger than NUM_SIGNATURES(20)"
    );
    assert!(
        index_window <= NUM_SIGNATURES,
        "index_window is bigger than NUM_SIGNATURES(20)"
    );
    assert!(
        index_window > offset_index,
        "index_window is less than or equal than offset_index"
    );

    let test_data = read_csv(testing_string);

    for entry_res in read_dir(training_string).expect("Failed to read directory") {
        let entry = entry_res.expect("Failed to process directory entry");
        let file_name = entry.file_name().to_str().unwrap().to_owned();

        if !file_name.starts_with(".") && entry.file_type().unwrap().is_dir() {
            let user_path = PathBuf::from(training_string).join(&file_name);

            let mut distance_array: [f32; NUM_SIGNATURES] = [0.0; NUM_SIGNATURES];

            for signature in offset_index..index_window {
                let signature_path =
                    user_path.join("signature_".to_owned() + &signature.to_string() + ".csv");
                let signature_string = signature_path.to_string_lossy().into_owned();

                let training_data = read_csv(&signature_string);

                let distance = dynamic_time_warping_multivariate(
                    test_data.0.as_slice(),
                    test_data.1,
                    test_data.2,
                    training_data.0.as_slice(),
                    training_data.1,
                    training_data.2,
                );

                distance_array[signature] = distance;
            }

            let metric = selector_function(&distance_array[offset_index..index_window]);

            if metric < threshold {
                if file_name != testing_user {
                    results.false_accept += 1;
                } else {
                    results.genuine_accept += 1;
                }
            } else {
                if file_name != testing_user {
                    results.genuine_reject += 1;
                } else {
                    results.false_reject += 1;
                }
            }
        }
    }
}

fn read_csv(file_path: &str) -> (Vec<f32>, usize, usize) {
    let file = File::open(file_path).unwrap();
    let reader = io::BufReader::new(file);

    let mut all_data: Vec<f32> = Vec::new();
    let mut num_rows = 0;
    let mut num_columns = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let data_string: Vec<&str> = line.split(',').collect();

        // Skip lines that don't split into multiple parts
        if data_string.len() == 1 {
            continue;
        }

        // Initialize num_columns on the first valid line
        if num_columns == 0 {
            num_columns = data_string.len();
        }

        // Parse the string data into floats
        let row: Result<Vec<f32>, _> = data_string.iter().map(|s| s.parse::<f32>()).collect();

        match row {
            Ok(row) => {
                all_data.extend(row);
                num_rows += 1;
            }
            Err(e) => eprintln!("Failed to parse row: {:?}", e),
        }
    }

    (all_data, num_rows, num_columns)
}
