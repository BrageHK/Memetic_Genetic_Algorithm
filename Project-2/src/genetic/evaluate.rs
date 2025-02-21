use crate::structs::config::Config;
use crate::structs::io::{Info, Patient};
use crate::structs::nurse::{Individual, Nurse};

use std::collections::HashMap;
use rayon::prelude::*;

pub fn fitness_population(
    population: &mut Vec<Individual>,
    info: &Info,
    config: &Config,
    fitness_hashmap: &HashMap<Vec<Nurse>, f32>,
) {
    population.par_iter_mut().for_each(|individual| {
        if let Some(&fitness_score) = fitness_hashmap.get(&individual.nurses) {
            individual.fitness = fitness_score;
        } else {
            let fitness_score: f32 = individual.nurses.par_iter()
                .map(|nurse| fitness_nurse(nurse, info, config))
                .sum();
            individual.fitness = fitness_score;
        }
    });
}

// Fitness function that "accepts" infeasable solutions
pub fn fitness_nurse(nurse: &Nurse, info: &Info, config: &Config) -> f32 {
    if nurse.route.is_empty() {
        return 0.0
    }

    let mut fitness: f32 = 0.0;
    let mut travel_time: f32;
    let mut time_used: f32= 0.0;
    let mut curr_demand: u32 = 0;
    let mut patient: Patient;

    // All patients
    let mut prev_p_idx = 0;
    for p in &nurse.route {
        patient = info.patients[*p as usize];
        travel_time = info.travel_times[prev_p_idx][*p as usize];
        time_used += travel_time + patient.care_time as f32;
        fitness += travel_time;
        if (patient.end_time as f32) < (time_used + patient.care_time as f32) {
            // Pasienten har tid til å bli behandlet.
            if patient.start_time as f32 > time_used {
                time_used = patient.start_time as f32;
            }
        } else {
            // Pasienten har ikke tid til å bli behandlet
            fitness += travel_time * config.fitness_punishment_factor;
        }
        curr_demand += patient.demand;
        prev_p_idx = *p as usize;
    }

    // From last patient to depot
    fitness += info.travel_times[0][prev_p_idx];

    fitness
}


pub fn feasable_fitness_nurse(nurse: &Nurse, info: &Info, config: &Config) -> f32 {
    if nurse.route.is_empty() {
        return 0.0
    }

    let mut fitness: f32 = 0.0;
    let mut travel_time: f32;
    let mut time_used: f32= 0.0;
    let mut curr_demand: u32 = 0;
    let mut patient: Patient;

    // All patients
    let mut prev_p_idx = 0;
    for p in &nurse.route {
        if curr_demand >  info.capacity_nurse {
            return 0.0;
        }
        patient = info.patients[*p as usize];
        travel_time = info.travel_times[prev_p_idx][*p as usize];
        time_used += travel_time + patient.care_time as f32;
        fitness += travel_time;
        if (patient.end_time as f32) < (time_used + patient.care_time as f32) {
            // Pasienten har tid til å bli behandlet.
            if patient.start_time as f32 > time_used {
                time_used = patient.start_time as f32;
            }
        } else {
            // Pasienten har ikke tid til å bli behandlet
            fitness += travel_time * config.fitness_punishment_factor;
        }
        curr_demand += patient.demand;
        prev_p_idx = *p as usize;
    }

    // From last patient to depot
    fitness += info.travel_times[0][prev_p_idx];

    fitness
}

pub fn get_best_fitness_population(population: &Vec<Individual>) -> f32 {
    let mut best_fitness: f32 = f32::INFINITY;
    for individual in population {
        if individual.fitness < best_fitness {
            best_fitness = individual.fitness;
        }
    }
    best_fitness
}