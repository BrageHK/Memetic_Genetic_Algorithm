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
    pub travel_times_sorted: Vec<Vec<usize>>,

    #[serde(deserialize_with = "patients_to_vec")]
    pub patients: Vec<Patient>
}
#[derive(Deserialize, Serialize, Debug)]
pub struct InfoRaw {
    pub instance_name: String,
    pub nbr_nurses: u32,
    pub capacity_nurse: u32,
    pub benchmark: f32,
    pub depot: Depot,
    pub travel_times: Vec<Vec<f32>>,

    #[serde(deserialize_with = "patients_to_vec")]
    pub patients: Vec<Patient>
}

impl From<InfoRaw> for Info {
    fn from(raw: InfoRaw) -> Self {
        let mut travel_times_sorted: Vec<Vec<usize>> = Vec::new();

        for patient_num in 0..raw.travel_times.len() {
            let cloone = raw.travel_times[patient_num].clone();
            let mut travel_times_indexed: Vec<(usize, &f32)> = cloone.iter().enumerate().collect();
            travel_times_indexed.sort_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal));
            let mut sorted_times: Vec<usize> = travel_times_indexed.into_iter().map(|(index, _)| index).collect();
            sorted_times.remove(0);
            travel_times_sorted.push(sorted_times);
        }


        Info {
            instance_name: raw.instance_name,
            nbr_nurses: raw.nbr_nurses,
            capacity_nurse: raw.capacity_nurse,
            benchmark: raw.benchmark,
            depot: raw.depot,
            travel_times: raw.travel_times,
            travel_times_sorted, // Populate the sorted travel times
            patients: raw.patients,
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Depot {
    pub return_time: u32,
    pub x_coord: i32,
    pub y_coord: i32
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
pub struct Patient {
    pub x_coord: i32,
    pub y_coord: i32,
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

pub fn read_from_json(config: &Config) -> Result<Info, Box<dyn Error>> {
    let f = File::open("train/".to_string() + &*config.file_name + ".json")?;
    let reader = BufReader::new(f);
    let info_raw: InfoRaw = from_reader(reader)?;
    let info = Info::from(info_raw);
    Ok(info)
}