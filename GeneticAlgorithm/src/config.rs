use serde::Deserialize;
use serde_yaml;
use std::fs::File;
use std::io::Read;
use csv::Reader;

#[derive(Debug, Deserialize)]
pub(crate) struct Config {
    pub population_size: i32,
    pub n_generations: i32,
    pub crossover_rate: f32,
    pub mutation_rate: f32,
    pub(crate) knapsack_capacity: u32,
    pub(crate) overweight_penalty_factor: f32,
}


pub fn load_config(path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let config: Config = serde_yaml::from_str(&contents)?;
    Ok(config)
}

#[derive(Debug)]
pub struct PW {
    pub(crate) p: u32,
    pub(crate) w: u32
}

pub fn get_data(path: &str) -> Vec<PW> {
    let file = File::open(path).unwrap();
    let mut reader = Reader::from_reader(file);
    let mut data: Vec<PW> = Vec::new();
    for record in reader.records() {
        let record = record.unwrap();
        data.push(PW{p: *&record[1].parse().unwrap(), w: *&record[2].parse().unwrap()})
    }
    data
}