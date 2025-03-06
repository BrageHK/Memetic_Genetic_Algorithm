use crate::structs::config::Config;
use crate::structs::io::{Info, Patient};
use crate::structs::nurse::{Individual, Nurse};

use std::collections::HashMap;
use ordered_float::OrderedFloat;
use rayon::prelude::*;

pub fn fitness_population(
    population: &mut Vec<Individual>,
    info: &Info,
    config: &Config,
    fitness_hashmap: &mut HashMap<Vec<Nurse>, f32>,
) {
    if config.use_islands {
        fitness_serial(population, &info, &config, fitness_hashmap);
    } else {
        fitness_parallel(population, &info, &config, fitness_hashmap);
    }
}

fn fitness_serial<'a>(
    population: &'a mut Vec<Individual>,
    info: &Info,
    config: &Config,
    fitness_hashmap: &mut HashMap<Vec<Nurse>, f32>,
) {
    // Compute fitness scores in parallel
    population
        .iter_mut()
        .for_each(|mut individual| {
            if let Some(&score) = fitness_hashmap.get(&individual.nurses) {
                individual.fitness = score;
            } else {
                let score: f32 = individual.nurses.par_iter()
                    .map(|nurse| fitness_nurse(nurse, info, config))
                    .sum();
                fitness_hashmap.insert(individual.nurses.clone(), score);
                individual.fitness = score;
            }
        });
}

pub fn fitness_parallel<'a>(
    population: &'a mut Vec<Individual>,
    info: &Info,
    config: &Config,
    fitness_hashmap: &mut HashMap<Vec<Nurse>, f32>,
) {
    // Compute fitness scores in parallel
    let new_fitness_scores: Vec<(Vec<Nurse>, f32)> = population
        .par_iter()
        .filter_map(|individual| {
            if let Some(&score) = fitness_hashmap.get(&individual.nurses) {
                Some((individual.nurses.clone(), score))
            } else {
                let score: f32 = individual.nurses.par_iter()
                    .map(|nurse| fitness_nurse(nurse, info, config))
                    .sum();
                Some((individual.nurses.clone(), score))
            }
        })
        .collect();

    // Update the shared HashMap
    for (nurses, score) in new_fitness_scores {
        fitness_hashmap.insert(nurses, score);
    }

    // Assign fitness scores to individuals_9
    for individual in population.iter_mut() {
        if let Some(&score) = fitness_hashmap.get(&individual.nurses) {
            individual.fitness = score;
        }
    }
}

pub fn fitness_nurse(nurse: &Nurse, info: &Info, config: &Config) -> f32 {
    if nurse.route.is_empty() {
        return 0.0
    }

    let mut fitness: f32 = 0.0;
    let mut time_used: f32 = 0.0;
    let mut curr_demand: u32 = 0;
    let mut patient: Patient;

    // All patients
    let mut prev_p_idx = 0;
    for p in &nurse.route {
        patient = info.patients[*p as usize];
        let p = p+1;
        let travel_time = info.travel_times[prev_p_idx][p as usize];
        fitness += travel_time;
        time_used += travel_time;
        if patient.start_time as f32 > time_used {
            time_used = patient.start_time as f32;
        }
        time_used += patient.care_time as f32;
        if (patient.end_time as f32) < time_used {
            // Pasienten har ikke tid til å bli behandlet -> punishment
            fitness += travel_time * config.fitness_punishment_factor;
        }
        curr_demand += patient.demand;
        prev_p_idx = p as usize;
    }
    if curr_demand > info.capacity_nurse {
        fitness += 1000.;
    }

    // From last patient to depot
    fitness += info.travel_times[0][prev_p_idx];
    time_used += info.travel_times[0][prev_p_idx];

    if time_used > info.depot.return_time as f32 {
        fitness += 3000.;
    }

    fitness
}

pub fn duration_demand_nurse(nurse: &Nurse, info: &Info) -> (u32, f32) {
    if nurse.route.is_empty() {
        return (0, 0.0);
    }

    let mut time_used: f32 = 0.0;
    let mut curr_demand: u32 = 0;

    // All patients
    let mut prev_p_idx = 0;
    for p in &nurse.route {
        let patient = info.patients[*p as usize];
        let p = p+1;
        let travel_time = info.travel_times[prev_p_idx][p as usize];
        time_used += travel_time;
        if patient.start_time as f32 > time_used {
            time_used = patient.start_time as f32;
        }
        time_used += patient.care_time as f32;
        curr_demand += patient.demand;
        prev_p_idx = p as usize;
    }

    // From last patient to depot
    time_used += info.travel_times[0][prev_p_idx];

    (curr_demand, time_used)
}

pub fn is_feasible_fitness_individual(individual: &Individual, info: &Info) -> bool {
    for nurse in &individual.nurses {
        if !is_feasible_fitness_nurse(nurse, &info) {
            return false;
        }
    }
    true
}

pub fn is_feasible_fitness_nurse(nurse: &Nurse, info: &Info) -> bool {
    if nurse.route.is_empty() {
        return true;
    }

    let mut time_used: f32 = 0.0;
    let mut curr_demand: u32 = 0;
    let mut patient: Patient;

    // All patients
    let mut prev_p_idx = 0;
    for p in &nurse.route {
        patient = info.patients[*p as usize];
        let p = p+1;

        // Travel
        time_used += info.travel_times[prev_p_idx][p as usize];

        // Wait
        if patient.start_time as f32 > time_used {
            time_used = patient.start_time as f32;
        }

        // Care
        time_used += patient.care_time as f32;

        curr_demand += patient.demand;
        prev_p_idx = p as usize;
        if (patient.end_time as f32) < time_used {
            return false;
        }
    }
    if curr_demand > info.capacity_nurse {
        return false;
    }
    time_used += info.travel_times[0][prev_p_idx] as f32;

    if time_used > info.depot.return_time as f32 {
        return false;
    }

    true
}

pub fn get_best_fitness_population(population: &Vec<Individual>) -> f32 {
    let best_individual = population.iter()
        .min_by(|a, b| a.fitness.partial_cmp(&b.fitness).unwrap());
    best_individual.unwrap().fitness
}

pub fn get_best_solution_population(population: &Vec<Individual>) -> Vec<Vec<i32>> {
    let best_individual = population
        .iter()
        .min_by_key(|i| OrderedFloat(i.fitness))
        .unwrap()
        .clone();

    let mut individual = Vec::new();
    for nurse in &best_individual.nurses {
        let incremented_route: Vec<i32> = nurse.route.iter().map(|&num| num + 1).collect();
        individual.push(incremented_route);
    }
    individual
}