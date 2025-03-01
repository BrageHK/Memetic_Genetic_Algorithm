use std::collections::HashSet;
use crate::structs::config::Config;
use crate::structs::nurse::{Individual};

pub fn get_elitism_members(population: &Vec<Individual>, config: &Config) -> Vec<Individual> {
    let mut members_hashset: HashSet<&Individual> = HashSet::new();
    for individual in population.iter().rev() {
        if let None = members_hashset.get(individual) {
            if members_hashset.len() >= config.n_elitism as usize {
                break;
            }
            members_hashset.insert(individual);
        }
    }
    members_hashset.into_iter().cloned().collect::<Vec<Individual>>()
}