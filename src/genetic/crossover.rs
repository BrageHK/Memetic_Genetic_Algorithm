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
        CrossoverFN::Visma =>  visma_crossover,
        CrossoverFN::VismaOptimized => visma_crossover_optimized,
        CrossoverFN::VismaIndexed => visma_crossover_heuristic,
        CrossoverFN::VismaMoreOptimized => visma_more_optimized,
    };

    // Could make this shorter, but don't care.
    let children: Vec<Individual>;
    if config.use_islands {
        children = parent_indices
            .chunks_exact(2)
            .flat_map(|parents| {
                let (parent1_idx, parent2_idx) = (parents[0], parents[1]);
                let (child_1, child_2) = crossover_fn(
                    &population[parent1_idx],
                    &population[parent2_idx],
                    &info,
                    &config
                );
                vec![child_1, child_2]
            })
            .collect();
    } else {
        children = parent_indices
            .par_chunks_exact(2)
            .flat_map(|parents| {
                let (parent1_idx, parent2_idx) = (parents[0], parents[1]);
                let (child_1, child_2) = crossover_fn(
                    &population[parent1_idx],
                    &population[parent2_idx],
                    &info,
                    &config
                );
                vec![child_1, child_2]
            })
            .collect();
    }

    children
}

fn visma_more_optimized(
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
    for _ in 0..5 {
        parent_idx_1 = rng.random_range(0..parent1.nurses.len());
        if !parent1.nurses[parent_idx_1].route.is_empty() {
            found = true;
            break;
        }
    }
    if !found { return (parent1.clone(), parent2.clone()); }

    let mut parent_idx_2: usize = 0;
    let mut found = false;
    for _ in 0..5 {
        parent_idx_2 = rng.random_range(0..parent2.nurses.len());
        if !parent2.nurses[parent_idx_2].route.is_empty() {
            found = true;
            break;
        }
    }
    if !found { return (parent1.clone(), parent2.clone()); }

    remove_crossover_optimized(&mut child1, &parent2, parent_idx_2);
    remove_crossover_optimized(&mut child2, &parent1, parent_idx_1);

    repair_nurse_optimized(&mut child1, &parent2, parent_idx_2, &info, &config);
    repair_nurse_optimized(&mut child2, &parent1, parent_idx_1, &info, &config);

    (child1, child2)
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
    for _ in 0..5 {
        parent_idx_1 = rng.random_range(0..parent1.nurses.len());
        if !parent1.nurses[parent_idx_1].route.is_empty() {
            found = true;
            break;
        }
    }
    if !found { return (parent1.clone(), parent2.clone()); }

    let mut parent_idx_2: usize = 0;
    let mut found = false;
    for _ in 0..5 {
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


fn visma_crossover_optimized(
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
    for _ in 0..5 {
        parent_idx_1 = rng.random_range(0..parent1.nurses.len());
        if !parent1.nurses[parent_idx_1].route.is_empty() {
            found = true;
            break;
        }
    }
    if !found { return (parent1.clone(), parent2.clone()); }

    let mut parent_idx_2: usize = 0;
    let mut found = false;
    for _ in 0..5 {
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

fn visma_crossover_heuristic(
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
    for _ in 0..5 {
        parent_idx_1 = rng.random_range(0..parent1.nurses.len());
        if !parent1.nurses[parent_idx_1].route.is_empty() {
            found = true;
            break;
        }
    }
    if !found { return (parent1.clone(), parent2.clone()); }

    let mut parent_idx_2: usize = 0;
    let mut found = false;
    for _ in 0..5 {
        parent_idx_2 = rng.random_range(0..parent2.nurses.len());
        if !parent2.nurses[parent_idx_2].route.is_empty() {
            found = true;
            break;
        }
    }
    if !found { return (parent1.clone(), parent2.clone()); }

    let repair_nurse_idx_1: usize = remove_crossover(&mut child1, &parent2, parent_idx_2);
    let repair_nurse_idx_2: usize = remove_crossover(&mut child2, &parent1, parent_idx_1);

    //repair_nurse_heuristic(&mut child1, &parent2, parent_idx_2, &info, &config);
    //repair_nurse_heuristic(&mut child2, &parent1, parent_idx_1, &info, &config);

    (child1, child2)
}

fn repair_nurse_optimized(
    to_repair: &mut Individual,
    parent: &Individual,
    parent_idx: usize,
    info: &Info,
    config: &Config,
) {
    for patient_to_insert_num in &parent.nurses[parent_idx].route {
        let mut lowest_fitness_change = f32::INFINITY;
        let mut best_insertion_idx_nurse = 0;
        let mut best_insertion_idx_patient = 0;

        let closest_patients: &Vec<usize> = &info.travel_times_sorted[*patient_to_insert_num as usize];

        let mut n_patients_tried = 0;
        let mut closest_patient_idx = 0;

        'outer: for close_patient_num in closest_patients {
            let mut found = false;
            let mut nurse_idx = 0;
            let mut patient_idx = 0;

            for (n_idx, nurse) in to_repair.nurses.iter().enumerate() {
                if let Some(idx) = nurse.route.iter().position(|x| x == &(*close_patient_num as i32)) {
                    found = true;
                    patient_idx = idx;
                    nurse_idx = n_idx;
                    break;
                }
            }

            if found {
                let fitness_before = fitness_nurse(&to_repair.nurses[nurse_idx], &info, &config).0;
                n_patients_tried += 1;

                // Try inserting before the current patient
                to_repair.nurses[nurse_idx].route.insert(patient_idx, *patient_to_insert_num);
                let fitness_after = fitness_nurse(&to_repair.nurses[nurse_idx], &info, &config).0;
                if fitness_after - fitness_before < lowest_fitness_change {
                    lowest_fitness_change = fitness_after - fitness_before;
                    best_insertion_idx_nurse = nurse_idx;
                    best_insertion_idx_patient = patient_idx;
                }
                to_repair.nurses[nurse_idx].route.remove(patient_idx);

                // Try inserting after the current patient
                if patient_idx + 1 <= to_repair.nurses[nurse_idx].route.len() {
                    to_repair.nurses[nurse_idx].route.insert(patient_idx + 1, *patient_to_insert_num);
                    let fitness_after = fitness_nurse(&to_repair.nurses[nurse_idx], &info, &config).0;
                    if fitness_after - fitness_before < lowest_fitness_change {
                        lowest_fitness_change = fitness_after - fitness_before;
                        best_insertion_idx_nurse = nurse_idx;
                        best_insertion_idx_patient = patient_idx + 1;
                    }
                    to_repair.nurses[nurse_idx].route.remove(patient_idx + 1);
                }
            }
            closest_patient_idx += 1;

            if n_patients_tried > 3 {
                break 'outer;
            }
        }

        // Insert the patient into the best nurse at the best position
        to_repair
            .nurses[best_insertion_idx_nurse]
            .route
            .insert(best_insertion_idx_patient, *patient_to_insert_num);
    }
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
                let old_fitness = fitness_nurse(nurse, &info, &config).0;
                nurse.route.insert(insertion_idx, patient.0 as i32);
                let new_fitness = fitness_nurse(nurse, &info, &config).0;
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

    for (nurse_idx, nurse) in &mut individual_to_change.nurses.iter_mut().enumerate() {
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

fn remove_crossover_optimized(individual_to_change: &mut Individual, other_individual: &Individual, i: usize) {
    let patients_to_remove = &other_individual.nurses[i].route;

    for nurse in &mut individual_to_change.nurses {
        let mut indices_to_remove: Vec<usize> = Vec::new();
        for (patient_idx, patient_pos) in nurse.route.iter().enumerate() {
            if patients_to_remove.contains(patient_pos) {
                indices_to_remove.push(patient_idx);
            }
        }
        for &idx in indices_to_remove.iter().rev() {
            nurse.route.remove(idx);
        }
    }
}
