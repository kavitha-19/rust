use plotters::prelude::*;
use crate::csv_parser::TitanicData;

#[derive(Debug)]
pub struct NumericalStatistics {
    pub mean_age: f64,
    pub median_age: f64,
    pub std_age: f64,
    pub min_age: f64,
    pub max_age: f64,
}

#[derive(Debug)]
pub struct CategoricalStatistics {
    pub unique_sex_count: usize,
    pub mode_sex: String,
    pub unique_embarked_count: usize,
    pub mode_embarked: String,
}

pub fn generate_plot(data: &[TitanicData]) -> Result<(), Box<dyn std::error::Error>> {
    // Extract survival and gender data for the bar chart
    let survival_and_gender: Vec<_> = data.iter().map(|d| (d.survived, &d.sex)).collect();

    // Specify the relative path to the src directory
    let file_path = "src/survival_by_gender.png";

    // Create a bar chart
    let root = BitMapBackend::new(file_path, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Survival Analysis by Gender", ("Arial", 20).into_font())
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0.0..2.0, 0.0..1.0)?;

    chart
        .configure_mesh()
        .disable_x_mesh()
        .disable_y_mesh()
        .y_labels(2)
        .draw()?;

    chart.draw_series(
        survival_and_gender
            .iter()
            .map(|(survived, gender)| {
                let x = if *survived == 1 { 1.0 } else { 0.0 };
                let y = if gender == &"female" { 1.0 } else { 0.0 };
                Rectangle::new([(x, y), (x + 0.5, y + 0.5)], BLUE.filled())
            }),
    )?;

    Ok(())
}


pub fn survival_analysis_by_gender(data: &[TitanicData]) -> Result<(), Box<dyn std::error::Error>> {
    // calculating survival rates for different genders
    let total_passengers = data.len();
    let female_survivors = data.iter().filter(|&d| d.survived == 1 && d.sex == "female").count();
    let male_survivors = data.iter().filter(|&d| d.survived == 1 && d.sex == "male").count();

    //println!("Survival Analysis by Gender:");
    println!("Female Survival Rate: {:.2}%", (female_survivors as f64 / total_passengers as f64) * 100.0);
    println!("Male Survival Rate: {:.2}%", (male_survivors as f64 / total_passengers as f64) * 100.0);

    Ok(())
}
