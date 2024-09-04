use std::env;
use std::fs::{read_dir, File};
use std::io::{self, BufRead};
use std::path::PathBuf;
use plotters::prelude::*;
use std::str;

//TODO: add treshold [v]
//TODO: modificare codice in modo tale da contare FA FR GA GR [v]
//TODO: calcolare FAR e FRR [v]
//TODO: iterare diversi treshold e stabilire ERR quando FAR e FRR sono uguali [v]
//TODO: make ROC graph [ ]
//TODO: adjust code based on DeMarsico's paper [ ]

const NUM_SIGNATURES: u8 = 20;
const NUM_USERS: u32 = 20;
const MAX_THRESHOLD: usize = 500;
const THRESHOLD_STEP: usize = 20;

struct TestResult {
    false_accept: u32,
    false_reject: u32,
    genuine_accept: u32,
    genuine_reject: u32,
}

fn normalize_tuples(tuples: &mut Vec<(f32, f32)>) {
    if tuples.is_empty() {
        return;
    }

    let mut min_val_x = f32::MAX;
    let mut max_val_x = f32::MIN;

    // Find min and max values for x
    for &(x, _) in tuples.iter() {
        min_val_x = min_val_x.min(x);
        max_val_x = max_val_x.max(x);
    }

    let mut min_val_y = f32::MAX;
    let mut max_val_y = f32::MIN;

    // Find min and max values for x
    for &(_, y) in tuples.iter() {
        min_val_y = min_val_y.min(y);
        max_val_y = max_val_y.max(y);
    }

    let range_x = max_val_x - min_val_x;
    let range_y = max_val_y - min_val_y;

    // Normalize each tuple
    for (x, y) in tuples.iter_mut() {
        *x = (*x - min_val_x) / range_x;
        *y = (*y - min_val_y) / range_y;
    }
}

fn plot_points(far_points: Vec<(f32,f32)>, frr_points: Vec<(f32,f32)>) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("prova.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;
    let root = root.margin(10, 10, 10, 10);
    // After this point, we should be able to construct a chart context
    let mut chart = ChartBuilder::on(&root)
        // Set the caption of the chart
        .caption("FAR & FRR", ("sans-serif", 10).into_font())
        // Set the size of the label region
        .x_label_area_size(20)
        .y_label_area_size(40)
        // Finally attach a coordinate on the drawing area and make a chart context
        .build_cartesian_2d(0f32..1f32, 0f32..1f32)?;

    // Then we can draw a mesh
    chart
        .configure_mesh()
        // We can customize the maximum number of labels allowed for each axis
        .x_labels(5)
        .y_labels(5)
        // We can also change the format of the label text
        .y_label_formatter(&|x| format!("{:.3}", x))
        .draw()?;

    // draw the points for the far values
    chart.draw_series(LineSeries::new(
        far_points,
        &BLUE,
    ))?
    .label("FAR")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], BLUE.filled()));

    // draw the points for the frr values
    chart.draw_series(LineSeries::new(
        frr_points,
        &RED,
    ))? 
    .label("FRR")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], RED.filled()));

    chart.configure_series_labels().draw()?;
    
    root.present()?;
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // Check if the correct number of arguments is given
    if args.len() != 2 {
        eprintln!("Usage: program <directory_path>",);
        std::process::exit(1);
    }

    let dir_path = &args[1];

    let mut far_points: Vec<(f32, f32)> = Vec::new();
    let mut frr_points: Vec<(f32, f32)> = Vec::new();
    
    let mut index = 0;
    for threshold in (0..=MAX_THRESHOLD).rev().step_by(THRESHOLD_STEP) {

        let mut results = TestResult {
            false_accept: 0,
            false_reject: 0,
            genuine_accept: 0,
            genuine_reject: 0,
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

                dtw_on_signature(
                    dir_path,
                    &testing_path.to_string_lossy(),
                    threshold as f32,
                    &test_file_name,
                    &mut results,
                );
            }
        }

        // println!("FA: {}, FR: {}, GA: {}, GR: {}", results.false_accept, results.false_reject, results.genuine_accept, results.genuine_reject);

        let rev_threshold = (MAX_THRESHOLD - (MAX_THRESHOLD - index * THRESHOLD_STEP)) as f32;
        let far_tuple = (rev_threshold, (results.false_accept/(NUM_USERS - 1)) as f32);
        let frr_tuple = (rev_threshold, results.false_reject as f32);

        far_points.push(far_tuple);
        frr_points.push(frr_tuple);
        
        index += 1;
    }

    normalize_tuples(&mut far_points);
    normalize_tuples(&mut frr_points);

    for point in &frr_points {
        println!("{} {}", point.0, point.1);
    }

    plot_points(far_points, frr_points).unwrap();
}

fn dtw_on_signature(
    training_string: &str,
    testing_string: &str,
    threshold: f32,
    testing_user: &str,
    results: &mut TestResult,
) {
    let test_data = read_csv(testing_string);

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

            let average_distance = distance_accumulator / NUM_SIGNATURES as f32;

            if average_distance < threshold {
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
