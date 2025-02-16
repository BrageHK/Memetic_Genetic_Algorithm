use serde::{Deserialize, Serialize, Deserializer};

use serde_json;
use serde_json::from_reader;

use std::error::Error;
use std::io::BufReader;
use std::fs::File;
use std::collections::HashMap;

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
    let patients_map: HashMap<String, Patient> = HashMap::deserialize(deserializer)?;
    Ok(patients_map.into_values().collect())
}

pub fn read_from_json(path: &str) -> Result<Info, Box<dyn Error>>{
    let f = File::open(path)?;
    let reader = BufReader::new(f);
    let info = from_reader(reader)?;
    Ok(info)
}