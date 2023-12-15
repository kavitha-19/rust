use std::error::Error;
use std::fs::File;
use serde::Deserialize;


#[derive(Debug, Deserialize, Clone)]
pub struct TitanicData {
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

impl TitanicData {
    pub fn preprocess_data(records: &[TitanicData]) -> Vec<TitanicData> {


    records
        .iter()
        .map(|record| {
            let cabin = record.cabin.clone().unwrap_or_else(|| "Unknown".to_owned());
            let embarked = record.embarked.clone().unwrap_or_else(|| "Unknown".to_owned());
            let _age = record.age.unwrap_or_default();
            TitanicData {
                cabin: Some(cabin),
                embarked: Some(embarked),
                age :match record.age {
                    Some(a) if a.is_nan() => None,
                    Some(_) => record.age,
                    None => Some(0.0),
                },
                ..record.clone()
            }
        })
        .collect()
        
    }
}

pub fn load_csv(file_path: &str) -> Result<Vec<TitanicData>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);

    let records = rdr.deserialize::<TitanicData>().collect::<Result<Vec<_>, _>>()?;

    Ok(records)
}
