use std::sync::Mutex;
use crate::structs::io::{Info, Patient};
use crate::structs::nurse::Individual;
use crate::genetic::evaluate::fitness_nurse;
use crate::structs::config::{Config, CrossoverFN};

use rand::Rng;
use rand::rngs::ThreadRng;
use rayon::slice::ParallelSlice;
use rayon::iter::ParallelIterator;

type CrossoverFNType = fn(&Individual, &Individual, &Info, &Config) -> (Individual, Individual);

pub fn population_crossover(
    population: &mut Vec<Individual>,
    parent_indices: &Vec<usize>,
    info: &Info,
    config: &Config,
) -> Vec<Individual> {
    let crossover_fn: CrossoverFNType = match config.crossover_fn {
        CrossoverFN::Visma =>  visma_crossover
    };

    let mut children: Mutex<Vec<Individual>> = Mutex::new(Vec::new());

    parent_indices.par_chunks_exact(2).for_each(|parents| {
        let (parent1_idx, parent2_idx) = (parents[0], parents[1]);
        let (child_1, child_2) = crossover_fn(
            &population[parent1_idx],
            &population[parent2_idx],
            &info,
            &config
        );
        let mut guard = children.lock().unwrap();
        guard.extend([child_1, child_2]);
    });

    children.into_inner().unwrap()
}

fn visma_crossover(
    parent1: &Individual,
    parent2: &Individual,
    info: &Info,
    config: &Config
) -> (Individual, Individual) {
    let mut child1: Individual = parent1.clone();
    let mut child2: Individual = parent2.clone();
    let mut rng: ThreadRng = rand::rng();
    if rng.random_range(0.0..=1.) > config.crossover_rate {
        return (parent1.clone(), parent2.clone())
    }
    let mut parent_idx_1: usize = 0;
    let mut found = false;
    for i in 0..5 {
        parent_idx_1 = rng.random_range(0..parent1.nurses.len());
        if !parent1.nurses[parent_idx_1].route.is_empty() {
            found = true;
            break;
        }
    }
    if !found { return (parent1.clone(), parent2.clone()); }

    let mut parent_idx_2: usize = 0;
    let mut found = false;
    for i in 0..5 {
        parent_idx_2 = rng.random_range(0..parent2.nurses.len());
        if !parent2.nurses[parent_idx_2].route.is_empty() {
            found = true;
            break;
        }
    }
    if !found { return (parent1.clone(), parent2.clone()); }

    let repair_nurse_idx_1: usize = remove_crossover(&mut child1, &parent2, parent_idx_2);
    let repair_nurse_idx_2: usize = remove_crossover(&mut child2, &parent1, parent_idx_1);

    repair_nurse(&mut child1, &parent2, parent_idx_2, repair_nurse_idx_1, &info, &config);
    repair_nurse(&mut child2, &parent1, parent_idx_1, repair_nurse_idx_2, &info, &config);

    (child1, child2)
}

fn repair_nurse(
    to_repair: &mut Individual,
    parent: &Individual,
    parent_idx: usize,
    nurse_idx: usize,
    info: &Info,
    config: &Config,
) {
    let mut patients: Vec<(usize, Patient)> = Vec::new();
    for patient_idx in parent.nurses[parent_idx].route.iter() {
        patients.push((*patient_idx as usize, info.patients[*patient_idx as usize]));
    }
    patients.sort_by_key(|p| (p.1.start_time, p.1.end_time));

    // Iterate over all nurses to find the best insertion point for each patient
    for patient in &patients {
        let mut lowest_fitness_change = f32::INFINITY;
        let mut best_nurse_idx = nurse_idx; // Start with the current nurse
        let mut best_insertion_idx = 0;

        // Check all nurses for the best insertion point
        for (n_idx, nurse) in to_repair.nurses.iter_mut().enumerate() {
            for insertion_idx in 0..=nurse.route.len() {
                let old_fitness = fitness_nurse(nurse, &info, &config);
                nurse.route.insert(insertion_idx, patient.0 as i32);
                let new_fitness = fitness_nurse(nurse, &info, &config);
                if new_fitness - old_fitness < lowest_fitness_change {
                    lowest_fitness_change = new_fitness - old_fitness;
                    best_nurse_idx = n_idx;
                    best_insertion_idx = insertion_idx;
                }
                nurse.route.remove(insertion_idx); // Revert the insertion for now
            }
        }

        // Insert the patient into the best nurse at the best position
        to_repair.nurses[best_nurse_idx]
            .route
            .insert(best_insertion_idx, patient.0 as i32);
    }
}

fn remove_crossover(individual_to_change: &mut Individual, other_individual: &Individual, i: usize) -> usize {
    let patients_to_remove = &other_individual.nurses[i].route;
    let mut insertion_nurses = Vec::new();

    for (nurse_idx, mut nurse) in &mut individual_to_change.nurses.iter_mut().enumerate() {
        let mut indices_to_remove: Vec<usize> = Vec::new();

        for (patient_idx, patient_pos) in nurse.route.iter().enumerate() {
            if patients_to_remove.contains(patient_pos) {
                indices_to_remove.push(patient_idx);
                if let Some((_, count)) = insertion_nurses.iter_mut().find(|(n, _)| *n == nurse_idx) {
                    *count += 1;
                } else {
                    insertion_nurses.push((nurse_idx, 1));
                }
            }
        }
        for &idx in indices_to_remove.iter().rev() {
            nurse.route.remove(idx);
        }
    }
    insertion_nurses.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap().0
}
