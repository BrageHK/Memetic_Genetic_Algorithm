use rand::prelude::ThreadRng;
use crate::structs::config::Config;
use crate::structs::nurse::Nurse;

use rand::Rng;
use rand::seq::SliceRandom;

pub fn mutate(population: &mut Vec<Nurse>, config: &Config) {
    let mut rng: ThreadRng= rand::rng();

    for _ in 0..config.mutation_loops {
        if rng.random_range(0.0..1.0) < config.inter_swap_mutation_rate  {
            inter_swap_mutation(population, &mut rng);
        }
        if rng.random_range(0.0..1.0) < config.inter_swap_mutation_rate  {

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
    let nurse_len = nurses.len();
    let mut nurse_1: &mut Nurse = nurses.get_mut(nurse_i).unwrap();

    let mut nurse_j;
    let mut nurse_2;
    loop {
        nurse_j = rng.random_range(0..nurse_len);
        if nurse_i != nurse_j {
            nurse_2 = nurses.get_mut(nurse_j).unwrap();
            break;
        }
    }

    if !nurse_1.route.is_empty() && !nurse_2.route.is_empty() {
        let nurse_i = rng.random_range(0..nurse_1.route.len());
        let nurse_j = rng.random_range(0..nurse_2.route.len());

        let patient_1 = nurse_1.route.get(nurse_i).unwrap();
        let patient_2 = nurse_2.route.get(nurse_j).unwrap();

        nurse_1.route.remove(nurse_i);
        nurse_1.route.insert(nurse_i, *patient_1);

        nurse_2.route.remove(nurse_j);
        nurse_2.route.insert(nurse_j, *patient_2);
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
    let mut nurse_1: &mut Nurse = nurses.get_mut(nurse_i).unwrap();

    let mut nurse_j;
    let mut nurse_2;
    loop {
        nurse_j = rng.random_range(0..nurses.len());
        if nurse_i != nurse_j {
            nurse_2 = nurses.get_mut(nurse_j).unwrap();
            break;
        }
    }

    if !nurse_1.route.is_empty() && !nurse_2.route.is_empty() {
        let patient_i = rng.random_range(0..nurse_1.route.len());
        let patient = nurse_1.route.get(patient_i).unwrap();
        nurse_1.route.remove(patient_i);

        let patient_j = rng.random_range(0..nurse_2.route.len());
        nurse_2.route.insert(patient_j, *patient);
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
