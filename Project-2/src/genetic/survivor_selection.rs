//NODE: NO DUPLICATE SOLUTIONS

use std::collections::{HashSet};
use rand::{rng, Rng};
use rayon::prelude::*;
use crate::structs::config::{Config, SurvivorSelectionFN};
use crate::structs::nurse::Individual;

//type SurvivorSelectionFNType = fn(&mut Vec<Individual>, &Vec<Individual>, &Config);

pub fn survivor_selection(
    population: &mut Vec<Individual>,
    parent_indices: &Vec<usize>,
    children: &Vec<Individual>,
    config: &Config
) {
    match config.survivor_selection_fn {
        SurvivorSelectionFN::Crowding => crowding(population, &children, &config),
        SurvivorSelectionFN::CrowdingOptimized => crowding_optimized(population, &children, &config, &parent_indices)
    };

}

// With the right compile flags, this is faster than Hashmap version
pub fn similarity(
    individual1: &Individual,
    individual2: &Individual,
    pop_size: usize
) -> f32 {
    let mut similarity = 0;
    for nurse1 in &individual1.nurses {
        if nurse1.route.len() < 2 {
            continue;
        }
        // Check edges
        for i in 0..nurse1.route.len()-1 {
            let edge1 = (nurse1.route[i], nurse1.route[i+1]);
            // Check if other individual contains these edges
            for nurse2 in &individual2.nurses {
                if nurse2.route.len() < 2 {
                    continue;
                }
                for j in 0..nurse2.route.len()-1 {
                    let edge2 = (nurse2.route[j], nurse2.route[j+1]);
                    if edge1 == edge2 {
                        similarity += 1;
                    }
                }
            }
        }
    }

    similarity as f32 / pop_size as f32
}

pub fn similarity_hashmap(individual1: &Individual, individual2: &Individual, pop_size: usize) -> f32 {
    let mut edge_hashmap: HashSet<(i32, i32)> = HashSet::new();
    let mut similarity = 0;

    for nurse in &individual1.nurses {
        for i in 0..nurse.route.len()-1 {
            if nurse.route.len() < 2 {
                continue;
            }
            edge_hashmap.insert((nurse.route[i], nurse.route[i+1]));
        }
    }

    for nurse in &individual2.nurses {
        for i in 0..nurse.route.len()-1 {
            if nurse.route.len() < 2 {
                continue;
            }
            if edge_hashmap.contains(&(nurse.route[i], nurse.route[i + 1])) {
                similarity += 1;
            }
        }
    }

    similarity as f32 / pop_size as f32
}

pub fn crowding_parallel(population: &mut Vec<Individual>, children: &Vec<Individual>, config: &Config) {
    population.par_iter().for_each(|individual| {

    });
    children.par_iter().for_each( |child| {
        let mut rng = rand::rng();
        let mut closest_index = 0;
        let mut closest_similarity_score = 0.0;

        for (i, individual) in population.iter().enumerate() {
            let similarity_score = similarity(&child, individual, population.len());
            if similarity_score > closest_similarity_score {
                closest_similarity_score = similarity_score;
                closest_index = i;
            }
        }
        let parent_fitness = population[closest_index].fitness;
        let child_fitness = child.fitness;
        let probability;
        if child_fitness > parent_fitness {
            probability = child_fitness / (child_fitness + config.scaling_factor * parent_fitness);
        } else if child.fitness == parent_fitness {
            probability = 0.5;
        } else {
            probability = (config.scaling_factor * child_fitness) / (config.scaling_factor * child_fitness + parent_fitness);
        }

        if rng.random_range(0.0..1.) < probability {
            //population[closest_index] = child.clone();
        }
    });
}
pub fn crowding(population: &mut Vec<Individual>, children: &Vec<Individual>, config: &Config) {
    let mut rng = rand::rng();
    for child in children {
        let mut closest_index = 0;
        let mut closest_similarity_score = 0.0;

        for (i, individual) in population.iter().enumerate() {
            let similarity_score = similarity(&child, individual, population.len());
            if similarity_score > closest_similarity_score {
                closest_similarity_score = similarity_score;
                closest_index = i;
            }
        }
        let parent_fitness = population[closest_index].fitness;
        let child_fitness = child.fitness;
        let probability;
        if child_fitness > parent_fitness {
            probability = child_fitness / (child_fitness + config.scaling_factor * parent_fitness);
        } else if child.fitness == parent_fitness {
            probability = 0.5;
        } else {
            probability = (config.scaling_factor * child_fitness) / (config.scaling_factor * child_fitness + parent_fitness);
        }

        if rng.random_range(0.0..1.) < probability {
            population[closest_index] = child.clone();
        }
    }
}

/// Single-threaded
pub fn crowding_optimized(
    population: &mut Vec<Individual>,
    children: &Vec<Individual>,
    config: &Config,
    parent_indices: &Vec<usize>,
) {
    parent_indices.chunks_exact(2).zip(children.chunks_exact(2))
        .for_each(|a| {
            let child1 = &a.1[0];
            let child2 = &a.1[1];
            let parent1_idx = a.0[0];
            let parent2_idx = a.0[1];

            if similarity(child1, &population[parent1_idx], population.len()) +
                similarity(child2, &population[parent2_idx], population.len()) <
                similarity(child1, &population[parent2_idx], population.len()) +
                    similarity(child2, &population[parent1_idx], population.len()) {
                compete(population, child1, parent1_idx, &config);
                compete(population, child2, parent2_idx, &config);
            } else {
                compete(population, child1, parent2_idx, &config);
                compete(population, child2, parent1_idx, &config);
            }
        });
}

fn compete(population: &mut Vec<Individual>, child: &Individual, parent_idx: usize, config: &Config) {
    let parent = &population[parent_idx];
    let child_fitness = child.fitness;
    let parent_fitness = parent.fitness;
    let probability;
    if child_fitness > parent_fitness {
        probability = child_fitness / (child_fitness + config.scaling_factor * parent_fitness);
    } else if child.fitness == parent_fitness {
        probability = 0.5;
    } else {
        probability = (config.scaling_factor * child_fitness) / (config.scaling_factor * child_fitness + parent_fitness);
    }

    if rng().random_range(0.0..1.) < probability {
        population[parent_idx] = child.clone();
    }
}