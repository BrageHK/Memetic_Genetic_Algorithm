use crate::structs::io::Info;
use crate::structs::nurse::Individual;
use crate::genetic::evaluate::fitness_nurse;
use crate::structs::config::Config;

use rand::Rng;
use rand::distr::Distribution;
use rand::distr::weighted::WeightedIndex;

fn visma_crossover(parent1: &mut Individual, parent2: &mut Individual, info: &Info, config: &Config) -> Individual {
    let mut parent1 = parent1.clone();
    let mut parent2 = parent2.clone();
    let mut rng = rand::rng();
    let nurse_idx_1: usize = rng.random_range(0..parent1.nurses.len());

    // Remove patients from parent 2
    for patient_idx_to_remove in &parent1.nurses[nurse_idx_1].route {
        let mut found = false;
        'outer: for mut nurse in parent2.nurses.iter_mut() {
            for patient_idx_parent2 in 0..nurse.route.len() {
                if nurse.route[patient_idx_parent2] == *patient_idx_to_remove {
                    nurse.route.remove(patient_idx_parent2);
                    found = true;
                    break 'outer;
                }
            }
        }
        if !found {
            panic!("Could not find route for patient_idx_to_remove. Something is wrong somewhere.");
        }
    }

    // Find best insertion for parent 2
    // Insert between each index of each nurse until best is found
    //TODO: slow??
    for patient_idx_to_add in &parent1.nurses[nurse_idx_1].route {
        let mut best_nurse_idx = 0;
        let mut best_route_idx = 0;
        let mut lowest_fitness = f32::INFINITY;

        for (nurse_idx, nurse) in parent2.nurses.iter_mut().enumerate() {
            for patient_idx_parent2 in 0..=nurse.route.len() {
                let fitness= fitness_nurse(nurse, info, config);
                nurse.route.insert(patient_idx_parent2, *patient_idx_to_add);
                let fitness_before = fitness_nurse(nurse, info, config);
                if fitness - fitness_before < lowest_fitness {
                    lowest_fitness = fitness;
                    best_nurse_idx = nurse_idx;
                    best_route_idx = patient_idx_parent2;
                }
                nurse.route.remove(patient_idx_parent2);
            }
        }

        parent2.nurses[best_nurse_idx].route.insert(best_route_idx, *patient_idx_to_add);
    }
    parent2
}

pub fn population_crossover(population: &mut Vec<Individual>, info: &Info, config: &Config) -> Vec<Individual> {
    let sum: f32 = population.iter().map(|individual| individual.fitness).sum();

    let probabilities = population
        .iter()
        .map(|individual| individual.fitness/sum)
        .collect::<Vec<f32>>();

    let mut dist = WeightedIndex::new(&probabilities).unwrap();

    let mut rng = rand::rng();

    let mut new_population: Vec<Individual> = Vec::new();

    for _ in 0..(population.len() - config.n_elitism as usize)/2 {
        let idx_parent_1 = dist.sample(&mut rng);
        let mut idx_parent_2 = 0;
        loop {
            idx_parent_2 = dist.sample(&mut rng);
            if idx_parent_1 != idx_parent_2 {
                break;
            }
        }

        let (slice1, slice2) = if idx_parent_1 < idx_parent_2 {
            let (left, right) = population.split_at_mut(idx_parent_2);
            (&mut left[idx_parent_1], &mut right[0])
        } else {
            let (left, right) = population.split_at_mut(idx_parent_1);
            (&mut right[0], &mut left[idx_parent_2])
        };

        let child_1 = visma_crossover(slice1, slice2, &info, &config);
        let child_2 = visma_crossover(slice2, slice1, &info, &config);

        new_population.push(child_1);
        new_population.push(child_2);
    }

    new_population
}