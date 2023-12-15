mod csv_parser;
mod statistics;
mod visualization;
mod original;
mod scattervisual;


use original::load_original_csv;
use csv_parser::load_csv;
use statistics::{
    calculate_numerical_statistics, calculate_categorical_statistics, survival_analysis_by_pclass,
};
use visualization::{generate_plot, survival_analysis_by_gender};
use scattervisual::{generate_scatterplot};
fn main() {
    // csv file
    let file_path = "titanic.csv";


    // Loading the CSV data (original data)
    match load_original_csv(file_path) {
        Ok(original_data) => {
            // Print the original data
            println!("Original Data:");
            for record in &original_data[..5] {
                // println!("{:?}", record);
                let embarked = record.embarked.as_deref().unwrap_or_default();
                let age = record.age.unwrap_or_default();
                let cabin = record.cabin.as_deref().unwrap_or_default();
                println!(
                    "OriginalTitanicData {{ passenger_id: {}, survived: {}, pclass: {}, name: \"{}\", sex: \"{}\", age: {}, sib_sp: {}, parch: {}, ticket: \"{}\", fare: {}, cabin: \"{}\", embarked: \"{}\" }}",
                    record.passenger_id, record.survived, record.pclass, record.name, record.sex, age, record.sib_sp, record.parch, record.ticket, record.fare, cabin, embarked
                );
            }
            

            
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    
    match load_csv(file_path){
        Ok(original_data) => {
    
           // Preprocess the CSV data
            let preprocessed_data = csv_parser::TitanicData::preprocess_data(&original_data);

            //Printing the first few records after preprocessing
            println!("\nPreprocessed Data:");
            for record in &preprocessed_data[..5] {
                println!("{:?}", record);
            }

            //Calculate and display numerical statistics
            let numerical_statistics = calculate_numerical_statistics(&preprocessed_data);
            println!("\nNumerical Statistics:");
            println!("{:?}", numerical_statistics);

            // Calculate and display categorical statistics
            let categorical_statistics = calculate_categorical_statistics(&preprocessed_data);
            println!("\nCategorical Statistics:");
            println!("{:?}", categorical_statistics);

            // Generate and save the plot
            if let Err(e) = generate_plot(&preprocessed_data) {
                eprintln!("Error generating plot: {}", e);
            }
            if let Err(e) = generate_scatterplot(&preprocessed_data) {
                eprintln!("Error generating plot: {}", e);
            }


            // Perform survival analysis by passenger class
            println!("\nSurvival Analysis by Passenger Class:");
            survival_analysis_by_pclass(&preprocessed_data);

            
            // Perform survival analysis by gender
            println!("\nSurvival Analysis by Gender:");
            if let Err(e) = survival_analysis_by_gender(&preprocessed_data) {
                eprintln!("Error performing survival analysis by gender: {}", e);
            }
           
        }
       
        Err(e) => eprintln!("Error: {}", e)
    } 
   
}
