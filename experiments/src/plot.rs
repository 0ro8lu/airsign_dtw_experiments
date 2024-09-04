use core::iter::zip;
use plotters::prelude::*;

pub fn plot_far_frr(
    far_points: Vec<(f32, f32)>,
    frr_points: Vec<(f32, f32)>,
) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("far&frr.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;
    let root = root.margin(10, 10, 10, 10);
    // After this point, we should be able to construct a chart context
    let mut chart = ChartBuilder::on(&root)
        // Set the caption of the chart
        .caption("FAR & FRR", ("sans-serif", 20).into_font())
        // Set the size of the label region
        .x_label_area_size(40)
        .y_label_area_size(40)
        // Finally attach a coordinate on the drawing area and make a chart context
        .build_cartesian_2d(0f32..1f32, 0f32..1f32)?;

    // let text_style = ("sans-serif", 50, &BLACK).into_text_style(&drawing_area);
    
    // Then we can draw a mesh
    chart
        .configure_mesh()
        // We can customize the maximum number of labels allowed for each axis
        .x_labels(5)
        .y_labels(5)
        .x_desc("Threshold")
        .y_desc("Error Rate")
        .x_label_style(("sans-serif", 16, &BLACK))
        .y_label_style(("sans-serif", 16, &BLACK))
        // We can also change the format of the label text
        // .y_label_formatter(&|x| format!("{:.1}", x))
        .draw()?;

    // draw the points for the far values
    chart
        .draw_series(LineSeries::new(far_points, &BLUE))?
        .label("FAR")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], BLUE.filled()));

    // chart
    //     .configure_series_labels()
    
    // draw the points for the frr values
    chart
        .draw_series(LineSeries::new(frr_points, &RED))?
        .label("FRR")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], RED.filled()));

    chart.configure_series_labels().label_font(("sans-serif", 16)).draw()?;

    root.present()?;
    Ok(())
}

pub fn plot_roc(
    gar_points: Vec<(f32, f32)>,
    far_points: Vec<(f32, f32)>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut roc_points: Vec<(f32, f32)> = Vec::new();

    for element in zip(gar_points, far_points) {
        roc_points.push((element.0 .1, element.1 .1));
    }

    let root = BitMapBackend::new("roc.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;
    let root = root.margin(10, 10, 10, 10);
    // After this point, we should be able to construct a chart context
    let mut chart = ChartBuilder::on(&root)
        // Set the caption of the chart
        .caption("ROC", ("sans-serif", 20).into_font())
        // Set the size of the label region
        .x_label_area_size(40)
        .y_label_area_size(40)
        // Finally attach a coordinate on the drawing area and make a chart context
        .build_cartesian_2d(0.0f32..1f32, 0f32..1f32)?;

    // Then we can draw a mesh
    chart
        .configure_mesh()
        // We can customize the maximum number of labels allowed for each axis
        .x_labels(5)
        .y_labels(5)
        .x_desc("FAR")
        .y_desc("GAR(1-FRR)")
        .x_label_style(("sans-serif", 16, &BLACK))
        .y_label_style(("sans-serif", 16, &BLACK))
        // We can also change the format of the label text
        // .y_label_formatter(&|x| format!("{:.3}", x))
        .draw()?;

    // draw the points for the frr values
    chart
        .draw_series(LineSeries::new(
            roc_points, // gar_points,
            &BLUE,
        ))?
        .label("ROC")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], BLUE.filled()));

    chart.configure_series_labels().label_font(("sans-serif", 16)).draw()?;

    root.present()?;
    Ok(())
}
