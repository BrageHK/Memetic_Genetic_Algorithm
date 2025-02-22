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
    fitness_hashmap: &HashMap<Vec<Nurse>, f32>,
    generation_idx: i32,
) {
    population.par_iter_mut().for_each(|individual| {
        if let Some(&fitness_score) = fitness_hashmap.get(&individual.nurses) {
            individual.fitness = fitness_score;
        } else {
            let fitness_score: f32 = individual.nurses.par_iter()
                .map(|nurse| fitness_nurse(nurse, info, config, generation_idx))
                .sum();
            if fitness_score <= 10. {
                panic!("Hello");
            }
            individual.fitness = fitness_score;
        }
    });
}

// Fitness function that "accepts" infeasable solutions
pub fn fitness_nurse(nurse: &Nurse, info: &Info, config: &Config, generation_idx: i32) -> f32 {
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
            fitness += travel_time * config.fitness_punishment_factor + (generation_idx as f32/10.);
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