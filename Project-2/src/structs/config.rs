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
pub struct Config  {
    pub train_file_num: i32,
    pub population_size: i32,
    pub n_generations: i32,
    pub n_elitism: i32,
    pub crossover_rate: f32,
    pub mutation_loops: i32,
    pub inter_swap_mutation_rate: f32,
    pub cross_swap_mutation_rate: f32,
    pub inter_insert_mutation_rate: f32,
    pub cross_insert_mutation_rate: f32,
    pub scramble_mutation_rate: f32,
    pub scramble_len: i32,
    pub inversion_mutation_rate: f32,
    pub inversion_len: i32,
    pub initial_pop_function: InitialPopType,
    pub fitness_punishment_factor: f32
}

impl Config {
    pub fn new(path: &str) -> Self {
        let file_content = fs::read_to_string(path).expect("Failed to read file");
        serde_yaml::from_str(&file_content).unwrap()
    }
}