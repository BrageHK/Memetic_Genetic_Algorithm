use serde::{Deserialize, Serialize};
use serde_yaml;
use std::fs;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum InitialPopType {
    Feasible,
    File,
}

#[derive(Debug, Deserialize, Clone)]
pub enum ParentSelectionFN {
    LinearRanking,
    Probabilistic,
    Tournament,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum CrossoverFN {
    Visma,
    VismaOptimized,
    VismaIndexed,
    VismaMoreOptimized,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum SurvivorSelectionFN {
    CrowdingOld,
    Crowding,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum ScrambleFN {
    Delete,
    Keep,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Config  {
    pub use_islands: bool,
    pub island_share_frequency: i32,
    pub print_and_graph: bool,
    pub run_time: i32,
    pub log_frequency: i32,

    pub file_name: String,
    pub population_size: i32,
    pub n_generations: i32,
    pub n_elitism: i32,
    pub n_stagnations: i32,
    pub crossover_rate: f32,
    pub crossover_tries: i8,

    pub heuristic_cluster_mutation_rate: f32,
    pub heuristic_swap_mutation_rate: f32,
    pub heuristic_random_swap_mutation_rate: f32,
    pub insert_mutation_rate: f32,
    pub large_neighbourhood_mutation_rate: f32,

    pub init_population_fn: InitialPopType,
    pub parent_selection_fn: ParentSelectionFN,
    pub crossover_fn: CrossoverFN,
    pub survivor_selection_fn: SurvivorSelectionFN,
    pub scramble_fn: ScrambleFN,

    pub scaling_factor: f32,
    pub n_parents_scaling: f32,
    pub fitness_punishment_factor: f32,
    pub s: f32,

    pub tournament_size: i32,
}

impl Config {
    pub fn new(path: &str) -> Self {
        let file_content = fs::read_to_string(path).expect("Failed to read file");
        serde_yaml::from_str(&file_content).unwrap()
    }
}