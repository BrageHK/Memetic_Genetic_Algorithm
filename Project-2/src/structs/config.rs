use serde::{Deserialize, Serialize};
use serde_yaml;
use std::fs;
use std::fs::File;
use std::io::Read;

#[derive(Deserialize, Serialize, Debug)]
pub enum InitialPopType {
    Feasible,
}

#[derive(Debug, Deserialize)]
pub enum ParentSelectionFN {
    LinearRanking,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum CrossoverFN {
    Visma,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum SurvivorSelectionFN {
    Crowding,
}

#[derive(Debug, Deserialize)]
pub struct Config  {
    pub train_file_num: i32,
    pub population_size: i32,
    pub n_generations: i32,
    pub n_elitism: i32,
    pub n_stagnations: i32,
    pub crossover_rate: f32,
    pub mutation_loops: i32,
    pub heuristic_cluster_mutation_rate: f32,
    pub init_population_fn: InitialPopType,
    pub parent_selection_fn: ParentSelectionFN,
    pub crossover_fn: CrossoverFN,
    pub survivor_selection_fn: SurvivorSelectionFN,
    pub scaling_factor: f32,
    pub n_parents_scaling: f32,
    pub fitness_punishment_factor: f32,
    pub s: f32,
}

impl Config {
    pub fn new(path: &str) -> Self {
        let file_content = fs::read_to_string(path).expect("Failed to read file");
        serde_yaml::from_str(&file_content).unwrap()
    }
}