use crate::structs::config::Config;
use crate::structs::nurse::{Individual, Nurse};
use crate::genetic::evaluate::fitness_nurse;
use crate::genetic::parent_selection::linear_rank_probability;
use crate::structs::io::Info;

use rand::Rng;
use rand::distr::weighted::WeightedIndex;
use rand::distr::Distribution;
use rand::prelude::ThreadRng;

use rayon::prelude::*;
use crate::genetic::large_neighborhood::destroy_and_repair;

pub fn mutate_population(population: &mut Vec<Individual>, config: &Config, info: &Info) {
    if config.use_islands {
        mutate_serial(population, &config, &info);
    } else {
        mutate_parallel(population, &config, &info);
    }
}

fn mutate_parallel(population: &mut Vec<Individual>, config: &Config, info: &Info) {
    population.par_iter_mut().for_each(|individual| mutate_nurse(&mut individual.nurses, &info, &config));
}

fn mutate_serial(population: &mut Vec<Individual>, config: &Config, info: &Info) {
    population.iter_mut().for_each(|individual| mutate_nurse(&mut individual.nurses, &info, &config));
}

type MutationFN = fn(nurses: &mut Vec<Nurse>, rng: &mut ThreadRng, info: &Info, config: &Config);

pub fn mutate_nurse(individual: &mut Vec<Nurse>, info: &Info, config: &Config) {
    let mut rng: ThreadRng= rand::rng();

    let mutations: [(MutationFN, f32); 5] = [
        (heuristic_cluster_mutation, config.heuristic_cluster_mutation_rate),
        (swap_mutation, config.random_swap_mutation_rate),
        (heuristic_swap_mutation, config.heuristic_swap_mutation_rate),
        (heurisitc_random_cross_swap_mutation, config.heuristic_random_swap_mutation_rate),
        (destroy_and_repair, config.large_neighbourhood_mutation_rate),
    ];

    for mutation_pair in &mutations {
        if rng.random_range(0.0..1.0) < mutation_pair.1 {
            mutation_pair.0(individual, &mut rng, &info, &config);
        }
    }
}


pub fn heuristic_cluster_mutation(nurses: &mut Vec<Nurse>, rng: &mut ThreadRng, info: &Info, config: &Config) {
    let mut nurse_idx;
    loop {
        nurse_idx = rng.random_range(0..nurses.len());
        if nurses[nurse_idx].route.len() > 1 {
            break;
        }
    }

    let start_idx = rng.random_range(0..nurses[nurse_idx].route.len());
    let end_idx = rng.random_range(start_idx..nurses[nurse_idx].route.len());

    let mut lowest_fitness = f32::INFINITY;
    let mut best_nurse_idx = 0;
    let mut best_route_idx = 0;

    let cluster: Vec<i32> = nurses[nurse_idx].route.drain(start_idx..end_idx).collect();

    for (nurse_idx, nurse) in nurses.iter_mut().enumerate() {
        let before_fitness = fitness_nurse(&nurse, &info, &config);
        for route_idx in 0..nurse.route.len() {
            nurse.route.splice(route_idx..route_idx, cluster.clone().into_iter());
            let after_fitness = fitness_nurse(&nurse, &info, &config);
            nurse.route.drain(route_idx..route_idx+cluster.len());
            if after_fitness - before_fitness < lowest_fitness {
                lowest_fitness = after_fitness - before_fitness;
                best_nurse_idx = nurse_idx;
                best_route_idx = route_idx;
            }
        }
    }

    nurses[best_nurse_idx].route.splice(best_route_idx..best_route_idx, cluster);
}

fn swap_mutation(nurses: &mut Vec<Nurse>, rng: &mut ThreadRng, info: &Info, config: &Config) {
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

fn heuristic_swap_mutation(nurses: &mut Vec<Nurse>, rng: &mut ThreadRng, info: &Info, config: &Config) {
    // Select a random nurse with at least 2 patients
    let mut nurse_idx;
    loop {
        nurse_idx = rng.random_range(0..nurses.len());
        if nurses[nurse_idx].route.len() > 1 {
            break;
        }
    }

    let mut lowest_fitness = f32::INFINITY;
    let mut best_pos1 = 0;
    let mut best_pos2 = 0;

    let before_fitness = fitness_nurse(&nurses[nurse_idx], &info, &config);

    for i in 0..nurses[nurse_idx].route.len() {
        for j in 0..nurses[nurse_idx].route.len() {
            if i != j {
                nurses[nurse_idx].route.swap(i,j);
                let after_fitness = fitness_nurse(&nurses[nurse_idx], &info, &config);
                nurses[nurse_idx].route.swap(j,i);

                if after_fitness - before_fitness < lowest_fitness {
                    lowest_fitness = after_fitness - before_fitness;
                    best_pos1 = i;
                    best_pos2 = j;
                }
            }
        }
    }

    nurses[nurse_idx].route.swap(best_pos1, best_pos2);
}

struct SwapFitness {
    i: usize,
    j: usize,
    fitness: f32,
}
fn heurisitc_random_cross_swap_mutation(nurses: &mut Vec<Nurse>, rng: &mut ThreadRng, info: &Info, config: &Config) {
    let nurse_i = rng.random_range(0..nurses.len());
    let nurse_len = nurses.len();

    let mut nurse_j;
    loop {
        nurse_j = rng.random_range(0..nurse_len);
        if nurse_i != nurse_j {
            break;
        }
    }

    if nurses[nurse_i].route.is_empty() && nurses[nurse_j].route.is_empty() {
        return;
    }

    let mut fitnesses: Vec<SwapFitness> = Vec::new();

    for patient_i in 0..nurses[nurse_i].route.len() {
        for patient_j in 0..nurses[nurse_j].route.len() {
            let patient_1 = *nurses[nurse_i].route.get(patient_i).unwrap();
            let patient_2 = *nurses[nurse_j].route.get(patient_j).unwrap();

            // Swap
            nurses[nurse_i].route[patient_i] = patient_2;
            nurses[nurse_j].route[patient_j] = patient_1;

            let fitness = fitness_nurse(&nurses[nurse_i], &info, &config)
                + fitness_nurse(&nurses[nurse_j], &info, &config);

            if fitnesses.is_empty() {
                fitnesses.push(SwapFitness{i: patient_i, j: patient_j, fitness})
            } else {
                // Insertion sort insertion.
                let mut inserted = false;
                let mut insertion_idx = 0;
                for (i, s) in fitnesses.iter().enumerate() {
                    if s.fitness < fitness {
                        inserted = true;
                        insertion_idx = i;
                        break;
                    }
                }
                if !inserted {
                    fitnesses.push(SwapFitness{i: patient_i, j: patient_j, fitness});
                } else {
                    fitnesses.insert(insertion_idx, SwapFitness{i: patient_i, j: patient_j, fitness});
                }
            }

            // Swap back
            nurses[nurse_i].route[patient_i] = patient_1;
            nurses[nurse_j].route[patient_j] = patient_2;
        }
    }


    let mu = fitnesses.len();
    let swap;
    if mu == 0 {
        return;
    }
    if mu < 2 {
        swap = fitnesses.first().unwrap();
    } else {
        let probabilities: Vec<f32> = fitnesses
            .iter()
            .enumerate()
            .map(|a| linear_rank_probability(mu, config.s, a.0))
            .collect::<Vec<f32>>();

        let mut dist = WeightedIndex::new(&probabilities).unwrap();

        let swap_idx = dist.sample(rng);

        swap = &fitnesses[swap_idx];
    }

    // Do final swap
    let temp = nurses[nurse_i].route[swap.i];
    nurses[nurse_i].route[swap.i] = nurses[nurse_j].route[swap.j];
    nurses[nurse_j].route[swap.j] = temp;
}