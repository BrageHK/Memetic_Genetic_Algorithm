use crate::structs::io::{Info, Patient};
use crate::structs::nurse::{Individual, Nurse};
use crate::genetic::evaluate::is_feasible_fitness_nurse;
use crate::structs::config::{Config, InitialPopType};

use rand::Rng;
use rand::prelude::IndexedRandom;

type InitIndividualFN = fn(&Info, &Config) -> Individual;

pub fn init_population(info: &Info, config: &Config) -> Vec<Individual> {
    let init_fn: InitIndividualFN = match config.init_population_fn {
        InitialPopType::Feasible => feasible_init_individual
    };
    let mut population = Vec::new();
    for _ in 0..=config.population_size {
        population.push(init_fn(&info, &config))
    }
    population
}

pub(crate) fn feasible_init_individual(info: &Info, config: &Config) -> Individual {
    let mut rng = rand::rng();
    let patients = info.patients
        .iter()
        .clone()
        .enumerate()
        .collect::<Vec<(usize, &Patient)>>();
    let mut nurses: Vec<Nurse> = vec![Nurse::new(); info.nbr_nurses as usize];

    for (patient_idx, _patient) in &patients {
        let mut n = 0;
        'outer: loop {
            let nurse_idx = rng.random_range(0..nurses.len());
            let route_idx =  if !nurses[nurse_idx].route.is_empty() {rng.random_range(0..nurses[nurse_idx].route.len())} else { 0 };
            nurses[nurse_idx].route.insert(route_idx, *patient_idx as i32);

            if is_feasible_fitness_nurse(&nurses[nurse_idx], &info) {
                break 'outer;
            }
            nurses[nurse_idx].route.remove(route_idx);

            n += 1;
            if n > 8050 {
                panic!("Bruh: {:?}, \ninfo: {:?}", &nurses, _patient);
            }
        }
    }
    Individual{nurses, fitness: -9999.}
}