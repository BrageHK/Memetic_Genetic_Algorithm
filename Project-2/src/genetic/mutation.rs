use rand::prelude::ThreadRng;
use crate::structs::config::Config;
use crate::structs::nurse::{Individual, Nurse};

use rand::Rng;

use rayon::prelude::*;
use crate::genetic::evaluate::fitness_nurse;
use crate::structs::io::Info;

pub fn mutate_population(population: &mut Vec<Individual>, config: &Config, info: &Info) {
    population.par_iter_mut().for_each(|individual| mutate_nurse(&mut individual.nurses, &info, &config));
}

pub fn mutate_nurse(individual: &mut Vec<Nurse>, info: &Info, config: &Config) {
    let mut rng: ThreadRng= rand::rng();

    for _ in 0..config.mutation_loops {
        if rng.random_range(0.0..1.0) < config.heuristic_cluster_mutation_rate  {
            heuristic_cluster_mutation(individual, &mut rng, &info, &config);
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

fn chunk_split_mutation() {}
