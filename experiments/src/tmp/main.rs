use std::cmp::Ordering;
use std::env;
use std::fs::{read_dir, File};
use std::io::{self, BufRead};
use std::path::PathBuf;
use std::str;

const NUM_SIGNATURES: u8 = 5;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Check if the correct number of arguments is given

    if args.len() != 3 {
        eprintln!(
            "Usage: {} <testing_file_path> <training_directory_path>",
            args[0]
        );
        std::process::exit(1);
    }

    let testing_string = &args[1];
    let training_string = &args[2];

    let test_data = read_csv(testing_string);

    let mut distance_vector: Vec<(f32, String)> = Vec::new();

    for entry_res in read_dir(training_string).expect("Failed to read directory") {
        let entry = entry_res.expect("Failed to process directory entry");
        let file_name = entry.file_name().to_str().unwrap().to_owned();

        if !file_name.starts_with(".") && entry.file_type().unwrap().is_dir() {
            let user_path = PathBuf::from(training_string).join(&file_name);

            let mut distance_accumulator = 0.0;

            for signature in 0..NUM_SIGNATURES {
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

                distance_accumulator += distance;
            }

            let distance_average = distance_accumulator / NUM_SIGNATURES as f32;
            distance_vector.push((distance_average, user_path.to_string_lossy().into_owned()));
        }
    }

    for item in &distance_vector {
        println!("Average Distance: {}, Path: {}", item.0, item.1);
    }

    println!("");

    if let Some(min) = distance_vector.iter().min_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(Ordering::Equal)) {
        println!("Minimum Distance: {}, User Path: {}", min.0, min.1);
    } else {
        println!("No data available.");
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

pub fn dynamic_time_warping_multivariate(
    vector_x: &[f32],
    num_x_rows: usize,
    num_x_columns: usize,
    vector_y: &[f32],
    num_y_rows: usize,
    num_y_columns: usize,
) -> f32 {
    if num_x_columns != num_y_columns {
        panic!("The number of columns in X and Y must be the same for multivariate DTW.");
    }

    let width = num_x_rows + 1;
    let height = num_y_rows + 1;

    let mut cost_matrix: Vec<f32> = vec![f32::INFINITY; width * height];
    cost_matrix[0] = 0.0;

    //calculate DTW
    for y in 1..height {
        for x in 1..width {
            let x_index = (x - 1) * num_x_columns;
            let y_index = (y - 1) * num_y_columns;

            let cost = euclidean_distance(
                &vector_x[x_index..x_index + num_x_columns],
                &vector_y[y_index..y_index + num_y_columns],
            );

            let value_match = cost_matrix[(x - 1) + ((y - 1) * width)];
            let value_insertion = cost_matrix[(x - 1) + (y * width)];
            let value_deletion = cost_matrix[x + ((y - 1) * width)];

            let final_value = cost + (value_insertion.min(value_deletion)).min(value_match);
            cost_matrix[x + (y * width)] = final_value;
        }
    }

    let x1 = *cost_matrix.last().unwrap();
    x1
}

fn euclidean_distance(a: &[f32], b: &[f32]) -> f32 {
    a.iter()
        .zip(b.iter())
        .map(|(ai, bi)| (ai - bi).powi(2))
        .sum::<f32>()
        .sqrt()
}
