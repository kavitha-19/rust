use plotters::prelude::*;

use crate::csv_parser::TitanicData;



pub fn generate_scatterplot(data: &[TitanicData]) -> Result<(), Box<dyn std::error::Error>> {
    // Extract age and fare data for the scatter plot
    let age_and_fare: Vec<_> = data.iter().filter_map(|d| d.age.map(|age| (age, d.fare))).collect();

    // Specify the relative path to the src directory
    let file_path = "src/scatter_plot.png";

    // Create a scatter plot
    let root = BitMapBackend::new(file_path, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Age vs. Fare", ("Arial", 20).into_font())
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0.0..80.0, 0.0..600.0)?;

    chart
        .configure_mesh()
        .disable_x_mesh()
        .disable_y_mesh()
        .draw()?;

    chart.draw_series(
        age_and_fare
            .iter()
            .map(|(age, fare)| Circle::new((*age, *fare), 5, RED.filled().stroke_width(1))),
    )?;

    Ok(())
}