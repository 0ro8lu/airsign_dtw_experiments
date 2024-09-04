#[no_mangle]
pub extern "C" fn my_add(x: i32, y: i32) -> i32 {
    x + y
}

#[no_mangle]
pub extern "C" fn ffi_dynamic_time_warping(
    x_buffer: *const f32,
    num_x_rows: usize,
    num_x_columns: usize,
    y_buffer: *const f32,
    num_y_rows: usize,
    num_y_columns: usize,
) -> f32 {
    unsafe {
        // Convert the raw pointers to slices representing the entire datasets
        let x_data = std::slice::from_raw_parts(x_buffer, num_x_rows * num_x_columns);
        let y_data = std::slice::from_raw_parts(y_buffer, num_y_rows * num_y_columns);

        // Compute DTW using these slices
        dynamic_time_warping_multivariate(
            x_data,
            num_x_rows,
            num_x_columns,
            y_data,
            num_y_rows,
            num_y_columns,
        )
    }
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
