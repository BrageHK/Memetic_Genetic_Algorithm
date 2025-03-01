use std::fs;
use crate::structs::io::{Info, Patient};
use crate::structs::nurse::{Individual, Nurse};
use crate::genetic::evaluate::is_feasible_fitness_nurse;
use crate::structs::config::{Config, InitialPopType};

use rand::{rng, Rng};
use serde_json::from_str;

type InitIndividualFN = fn(&Info, &Config) -> Vec<Individual>;

pub fn init_population(info: &Info, config: &Config) -> Vec<Individual> {
    let init_fn: InitIndividualFN = match config.init_population_fn {
        InitialPopType::Feasible => feasible_pop,
        InitialPopType::File => get_population_from_file,
    };
    init_fn(&info, &config)
}

fn feasible_pop(info: &Info, config: &Config) -> Vec<Individual> {
    let mut pop = Vec::new();
    for _ in 0..config.population_size as  usize {
        pop.push(feasible_init_individual(&info, &config));
    }
    pop
}

pub(crate) fn feasible_init_individual(info: &Info, _config: &Config) -> Individual {
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

fn get_population_from_file(info: &Info, config: &Config) -> Vec<Individual> {
    let folder_path = "./individuals";

    let mut population: Vec<Vec<Vec<i32>>> = Vec::new();

    for entry in fs::read_dir(folder_path).unwrap() {
        let entry = entry.unwrap();
        let filename = entry.file_name();
        let fp = format!("{}/{}", folder_path, &filename.to_str().unwrap());
        let file_content = fs::read_to_string(&fp).unwrap();
        population.push(from_str(&file_content).unwrap());
    }

    let mut new_pop: Vec<Individual> = Vec::new();

    for individual in &population {
        let mut nurses = vec![Nurse::new(); info.nbr_nurses as usize];
        for nurse_idx in 0..info.nbr_nurses as usize {
            nurses[nurse_idx] = Nurse{route: individual[nurse_idx].clone(), capacity: 0 };
            for i in nurses[nurse_idx].route.iter_mut() {
                *i -= 1;
            }
        }
        new_pop.push(Individual{nurses, fitness: 0.})
    }

    if new_pop.len() < config.population_size as usize {
        for _ in 0..config.population_size as usize - new_pop.len() {
            let new_member = new_pop[rng().random_range(0..new_pop.len())].clone();
            new_pop.push(new_member);
        }
    } else if new_pop.len() > config.population_size as usize {
        for _ in 0..config.population_size as usize - new_pop.len() {
            new_pop.remove(rng().random_range(0..new_pop.len()));
        }
    }

    new_pop
}
