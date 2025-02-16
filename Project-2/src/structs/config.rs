use serde::{Deserialize, Serialize};
use serde_yaml;
use std::fs;
use std::fs::File;
use std::io::Read;

#[derive(Deserialize, Serialize, Debug)]
pub enum InitialPopType {
    Clustering,
    StartTime
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub population_size: i32,
    pub n_generations: i32,
    pub crossover_rate: f32,
    pub mutation_rate: f32,
    pub initial_pop_function: InitialPopType
}

impl Config {
    pub fn new(path: &str) -> Self {
        let file_content = fs::read_to_string(path).expect("Failed to read file");
        serde_yaml::from_str(&file_content).unwrap()
    }
}