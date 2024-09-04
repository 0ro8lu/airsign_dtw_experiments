extern crate hello_from_quest;

use std::fs::File;
use std::io::{self, BufRead};

use hello_from_quest::ffi_dynamic_time_warping;

fn main() {
    let result_0 =
        read_csv("/Users/macjack/Library/Application Support/DefaultCompany/AirSign(OpenXR)/user_0/signature_0.csv");

    let result_1 =
        read_csv("/Users/macjack/Library/Application Support/DefaultCompany/AirSign(OpenXR)/user_0/signature_1.csv");

    ffi_dynamic_time_warping(result_0.0.as_ptr(), 248, 9, result_1.0.as_ptr(), 234, 9)
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
