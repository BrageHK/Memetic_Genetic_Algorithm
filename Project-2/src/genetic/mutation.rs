use rand::prelude::ThreadRng;
use crate::structs::config::Config;
use crate::structs::nurse::{Individual, Nurse};

use rand::Rng;
use rand::seq::SliceRandom;

pub fn mutate_population(population: &mut Vec<Individual>, config: &Config) {
    for individual in population {
        mutate_nurse(&mut individual.nurses, config)
    }
}

pub fn mutate_nurse(individual: &mut Vec<Nurse>, config: &Config) {
    let mut rng: ThreadRng= rand::rng();

    for _ in 0..config.mutation_loops {
        if rng.random_range(0.0..1.0) < config.inter_swap_mutation_rate  {
            inter_swap_mutation(individual, &mut rng);
        }
        if rng.random_range(0.0..1.0) < config.cross_swap_mutation_rate  {
            cross_swap_mutation(individual, &mut rng);
        }
        if rng.random_range(0.0..1.0) < config.inter_insert_mutation_rate  {
            inter_insert_mutation(individual, &mut rng);
        }
        if rng.random_range(0.0..1.0) < config.cross_insert_mutation_rate  {
            cross_swap_mutation(individual, &mut rng);
        }
        if rng.random_range(0.0..1.0) < config.inversion_mutation_rate {
            inversion_mutation(individual, &mut rng, &config)
        }
        if rng.random_range(0.0..1.0) < config.inversion_mutation_rate {
            scramble_mutation(individual, &mut rng, &config)
        }
    }
}

fn inter_swap_mutation(nurses: &mut Vec<Nurse>, rng: &mut ThreadRng) {
    let nurse_idx = rng.random_range(0..nurses.len());
    let mut nurse: &mut Nurse = nurses.get_mut(nurse_idx).unwrap();

    if nurse.route.len() > 1 {
        let i = rng.random_range(0..nurse.route.len());
        let mut j;

        loop {
            j = rng.random_range(0..nurse.route.len());
            if j != i {
                break;
            }
        }

        nurse.route.swap(i, j);
    }
}

fn cross_swap_mutation(nurses: &mut Vec<Nurse>, rng: &mut ThreadRng) {
    let nurse_i = rng.random_range(0..nurses.len());
    let nurse_j = loop {
        let j = rng.random_range(0..nurses.len());
        if j != nurse_i {
            break j;
        }
    };

    if !nurses[nurse_i].route.is_empty() && !nurses[nurse_j].route.is_empty() {
        let route_i_index = rng.random_range(0..nurses[nurse_i].route.len());
        let route_j_index = rng.random_range(0..nurses[nurse_j].route.len());

        let patient_1 = nurses[nurse_i].route[route_i_index];
        let patient_2 = nurses[nurse_j].route[route_j_index];

        nurses[nurse_i].route[route_i_index] = patient_2;
        nurses[nurse_j].route[route_j_index] = patient_1;
    }
}

fn inter_insert_mutation(nurses: &mut Vec<Nurse>, rng: &mut ThreadRng) {
    let nurse_idx = rng.random_range(0..nurses.len());
    let mut nurse: &mut Nurse = nurses.get_mut(nurse_idx).unwrap();

    if nurse.route.len() > 1 {
        let i = rng.random_range(0..nurse.route.len());
        let route = nurse.route.remove(i);
        let j = rng.random_range(0..nurse.route.len());

        nurse.route.insert(j, route);
    }
}

fn cross_insert_mutation(nurses: &mut Vec<Nurse>, rng: &mut ThreadRng) {
    let nurse_i = rng.random_range(0..nurses.len());
    let mut nurse_j;
    loop {
        nurse_j = rng.random_range(0..nurses.len());
        if nurse_i != nurse_j {
            break;
        }
    }

    if !nurses[nurse_i].route.is_empty() && !nurses[nurse_j].route.is_empty() {
        let patient_i = rng.random_range(0..nurses[nurse_i].route.len());
        let patient = nurses[nurse_i].route[patient_i];
        nurses[nurse_i].route.remove(patient_i);

        let patient_j = rng.random_range(0..nurses[nurse_j].route.len());
        nurses[nurse_j].route.insert(patient_j, patient);
    }
}

fn scramble_mutation(nurses: &mut Vec<Nurse>, rng: &mut ThreadRng, config: &Config) {
    let nurse_idx = rng.random_range(0..nurses.len());
    let mut nurse: &mut Nurse = nurses.get_mut(nurse_idx).unwrap();

    if nurse.route.len() > config.scramble_len as usize {
        let start_idx = rng.random_range(0..=nurse.route.len()-config.scramble_len as usize);

        let mut scrambled: Vec<i32> = Vec::new();

        for route_idx in start_idx..start_idx+config.scramble_len as usize {
            scrambled.push(*nurse.route.get(route_idx).unwrap());
            nurse.route.remove(route_idx);
        }
        scrambled.shuffle(rng);

        for scrambled_patient in scrambled {
            nurse.route.insert(start_idx, scrambled_patient);
        }
    }
}

fn inversion_mutation(nurses: &mut Vec<Nurse>, rng: &mut ThreadRng, config: &Config) {
    let nurse_idx = rng.random_range(0..nurses.len());
    let mut nurse: &mut Nurse = nurses.get_mut(nurse_idx).unwrap();

    if nurse.route.len() > config.inversion_len as usize {
        let start_idx = rng.random_range(0..=nurse.route.len() - config.inversion_len as usize);
        let end_idx = start_idx + config.inversion_len as usize;

        let slice: Vec<i32> = nurse.route[start_idx..end_idx]
            .iter()
            .rev()
            .cloned()
            .collect();

        nurse.route.splice(start_idx..end_idx, slice);
    }
}
