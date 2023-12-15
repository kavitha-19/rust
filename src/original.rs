use std::error::Error;
use std::fs::File;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct OriginalTitanicData {
    pub passenger_id: i32,
    pub survived: i32,
    pub pclass: i32,
    pub name: String,
    pub sex: String,
    pub age: Option<f64>,
    pub sib_sp: i32,
    pub parch: i32,
    pub ticket: String,
    pub fare: f64,
    pub cabin: Option<String>,
    pub embarked: Option<String>,
}

pub fn load_original_csv(file_path: &str) -> Result<Vec<OriginalTitanicData>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);

    let records = rdr.deserialize::<OriginalTitanicData>().collect::<Result<Vec<_>, _>>()?;

    Ok(records)
}
