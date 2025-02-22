use serde::{Deserialize, Serialize, Deserializer};

use serde_json;
use serde_json::from_reader;

use std::error::Error;
use std::io::BufReader;
use std::fs::File;
use std::collections::HashMap;
use crate::structs::config::Config;

#[derive(Deserialize, Serialize, Debug)]
pub struct Info {
    pub instance_name: String,
    pub nbr_nurses: u32,
    pub capacity_nurse: u32,
    pub benchmark: f32,
    pub depot: Depot,
    pub travel_times: Vec<Vec<f32>>,

    #[serde(deserialize_with = "patients_to_vec")]
    pub patients: Vec<Patient>
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Depot {
    pub return_time: u32,
    pub x_coord: u32,
    pub y_coord: u32
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
pub struct Patient {
    pub x_coord: f32,
    pub y_coord: f32,
    pub demand: u32,
    pub start_time: u32,
    pub end_time: u32,
    pub care_time: u32
}

fn patients_to_vec<'de, D>(deserializer: D) -> Result<Vec<Patient>, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::Error;

    let patients_map: HashMap<String, Patient> = HashMap::deserialize(deserializer)?;

    let mut patients_vec: Vec<(usize, Patient)> = patients_map
        .into_iter()
        .map(|(key, patient)| {
            key.parse::<usize>()
                .map(|num| (num - 1, patient)) // Convert key to zero-based index
                .map_err(|_| D::Error::custom(format!("Invalid patient key: {}", key)))
        })
        .collect::<Result<Vec<_>, D::Error>>()?;

    // Sort by the parsed index
    patients_vec.sort_by_key(|&(index, _)| index);

    // Ensure the indices match expected values
    for (expected, (actual, _)) in patients_vec.iter().enumerate() {
        if *actual != expected {
            return Err(D::Error::custom(format!(
                "Patient keys are not sequential. Expected {}, got {}",
                expected + 1,
                actual + 1
            )));
        }
    }

    Ok(patients_vec.into_iter().map(|(_, patient)| patient).collect())
}

pub fn read_from_json(config: &Config) -> Result<Info, Box<dyn Error>>{
    let f = File::open("train/train_".to_string() + &*config.train_file_num.to_string() + ".json")?;
    let reader = BufReader::new(f);
    let info = from_reader(reader)?;
    Ok(info)
}