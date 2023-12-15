
use std::collections::HashMap;
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

// Functions to calculate statistics
pub fn calculate_numerical_statistics(data: &[TitanicData]) -> NumericalStatistics {
    // Calculate mean, median, standard deviation, min, and max for numerical features
    let ages: Vec<_> = data.iter().filter_map(|d| d.age).collect();
    let mean_age: f64 = ages.iter().sum::<f64>() / ages.len() as f64;
    let median_age: f64 = calculate_median(&ages);
    let std_age: f64 = calculate_standard_deviation(&ages, mean_age);
    let min_age: f64 = ages.iter().cloned().fold(f64::INFINITY, f64::min);
    let max_age: f64 = ages.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

    NumericalStatistics {
        mean_age,
        median_age,
        std_age,
        min_age,
        max_age,
    }
}

pub fn calculate_categorical_statistics(data: &[TitanicData]) -> CategoricalStatistics {
    // Calculate count of unique values, mode for categorical features
    let sex_binding = data.iter().map(|d| &d.sex).collect::<Vec<_>>();
    let sex_counts = count_unique_values(&sex_binding);
    
    let embarked_binding = data.iter().filter_map(|d| d.embarked.as_deref()).collect::<Vec<_>>();
    let embarked_counts = count_unique_values(&embarked_binding);

    //let mode_sex = find_mode(&sex_counts);
    //let mode_embarked = find_mode(&embarked_counts);

    let mode_sex = find_mode(&sex_counts);
    let mode_embarked = find_mode(&embarked_counts);



    CategoricalStatistics {
        unique_sex_count: sex_counts.len(),
        mode_sex,
        unique_embarked_count: embarked_counts.len(),
        mode_embarked,
    }
}

// Helper function to calculate median
fn calculate_median(data: &[f64]) -> f64 {
    let mut sorted_data = data.to_vec();
    sorted_data.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mid = sorted_data.len() / 2;
    if sorted_data.len() % 2 == 0 {
        (sorted_data[mid - 1] + sorted_data[mid]) / 2.0
    } else {
        sorted_data[mid]
    }
}

// Helper function to calculate standard deviation
fn calculate_standard_deviation(data: &[f64], mean: f64) -> f64 {
    let variance = data.iter().map(|&x| (x - mean).powi(2)).sum::<f64>() / data.len() as f64;
    variance.sqrt()
}

// Helper function to count unique values
fn count_unique_values<T: Eq + std::hash::Hash + std::fmt::Display>(data: &[T]) -> HashMap<String, usize> {

//fn count_unique_values<T: Eq + std::hash::Hash + std::fmt::Display>(data: &[T]) -> std::collections::HashMap<String, usize> {
    let mut counts = std::collections::HashMap::new();
    for value in data {
        *counts.entry(value.to_string()).or_insert(0) += 1;
    }
    counts
}


// Helper function to find mode
fn find_mode(counts: &HashMap<String, usize>) -> String {

//fn find_mode(counts: &std::collections::HashMap<String, usize>) -> String {
    counts.iter().max_by_key(|&(_, count)| count).map(|(value, _)| value.clone()).unwrap_or_default()
}



// Analysis 


pub fn survival_analysis_by_pclass(data: &[TitanicData]) {
    // Group data by passenger class
    let mut grouped_by_pclass: HashMap<i32, Vec<&TitanicData>> = HashMap::new();

    for record in data.iter().filter(|record| (1..=3).contains(&record.pclass)) {
        grouped_by_pclass
            .entry(record.pclass)
            .or_insert_with(Vec::new)
            .push(record);
    }

    
    for &pclass in &[1, 2, 3] {
        if let Some(group) = grouped_by_pclass.get(&pclass) {
            let total_passengers = group.len();
            let survivors = group.iter().filter(|&&record| record.survived == 1).count();
            let survival_rate = survivors as f64 / total_passengers as f64;
    
            println!(
                "Class {}: {} passengers, {} survivors, {:.2}% survived",
                pclass, total_passengers, survivors, survival_rate * 100.0
            );
        } else {
            println!("Class {}: No data available", pclass);
        }
    }
    
}
