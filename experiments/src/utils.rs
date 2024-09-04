use core::cmp::Ordering;

pub fn elements_min(elements: &[f32]) -> f32 {
    if let Some(min) = elements
        .iter()
        .min_by(|a, b| a.partial_cmp(&b).unwrap_or(Ordering::Equal))
    {
        *min
    } else {
        println!("No data available.");
        0.0
    }
}

pub fn elements_max(elements: &[f32]) -> f32 {
    if let Some(max) = elements
        .iter()
        .max_by(|a, b| a.partial_cmp(&b).unwrap_or(Ordering::Equal))
    {
        *max
    } else {
        println!("No data available.");
        0.0
    }
}

pub fn elements_avg(elements: &[f32]) -> f32 {
    let mut accumulator = 0.0;
    let mut counter = 0;

    for element in elements {
        counter += 1;
        accumulator += element;
    }

    accumulator / counter as f32
}

pub fn normalize_tuples(tuples: &mut Vec<(f32, f32)>) {
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
