use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::available_parallelism;
use std::time::Instant;

use rand::prelude::*;
use rand::distr::weighted::WeightedIndex;
use rand::rng;
use crate::genetic::crossover::population_crossover;
use crate::genetic::elitism::get_elitism_members;
use crate::genetic::evaluate::{fitness_population, get_best_fitness_population};
use crate::genetic::initialize_population::init_population;
use crate::genetic::mutation::mutate_population;
use crate::genetic::parent_selection::parent_selection;
use crate::genetic::scramble::scramble_population;
use crate::genetic::survivor_selection::survivor_selection;
use crate::structs::config::Config;
use crate::structs::io;
use crate::structs::nurse::Individual;
use crate::util::save_individual::save_individual;

pub(crate) fn start_islands(config: Config) {
    let num_islands: usize = available_parallelism().unwrap().get();
    let shared_individuals = Arc::new(Mutex::new(Vec::new()));
    let mut rng= rand::rng();

    println!("Creating {} islands!", num_islands);

    let mut handles = vec![];

    for _ in 0..num_islands {
        let shared_clone = Arc::clone(&shared_individuals);
        let config_clone = config.clone(); // Ensure Config is Clone

        let handle = thread::spawn(move || {
            islands(config_clone, shared_clone);
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

pub(crate) fn islands(config: Config, shared_individuals: Arc<Mutex<Vec<Individual>>>) {
    let info = io::read_from_json(&config).unwrap();
    let mut rng = rng();

    let mut population = init_population(&info, &config);

    fitness_population(&mut population, &info, &config);

    let mut stagnation_counter: i32 = 0;
    let mut best_fitness: f32 = f32::INFINITY;

    let start = Instant::now();

    for i in 0..config.n_generations {
        if config.run_time != -1 {
            if start.elapsed().as_secs() > config.run_time as u64 {
                break;
            }
        }
        population.sort_by(|p1, p2| p2.fitness.total_cmp(&p1.fitness));


        if i % config.island_share_frequency == 0 {
            // Compute inverse fitness values (lower fitness = higher probability)
            let mut fitness_weights: Vec<f32> = population
                .iter()
                .map(|ind| 1.0 / (ind.fitness)) // Avoid division by zero
                .collect();

            let sum: f32 = fitness_weights.iter().sum();
            for fitness_idx in 0..fitness_weights.len() {
                fitness_weights[fitness_idx] /= sum;
            }

            // Create weighted distribution
            if let Ok(weighted_dist) = WeightedIndex::new(&fitness_weights) {
                let selected_index = weighted_dist.sample(&mut rng);
                let selected_individual = population[selected_index].clone();

                let mut shared = shared_individuals.lock().unwrap();
                if !shared.is_empty() {
                    let migrant = shared.pop().unwrap();
                    population.push(migrant);
                }
                shared.push(selected_individual);
                population.remove(selected_index);
            }
        }
        population.sort_by(|p1, p2| p2.fitness.total_cmp(&p1.fitness));

        if i % 100 == 0 {
            let fitnesses: Vec<f32> = population.iter().map(|x| x.fitness).collect::<Vec<f32>>();
            let last_fitnesses = &fitnesses[fitnesses.len()-5..];
            let avg_fitness = fitnesses.iter().sum::<f32>() / fitnesses.len() as f32;
            println!("nGenerations: {} Best fitnesses: {:?} Avg Fitness: {} Execution_time {:?}", i, last_fitnesses, &avg_fitness, &start.elapsed());
        }
        /* if i % 10 == 0 { print!("Fitnesses: ["); for individual in population.iter() { print!("({}, {}),", individual.fitness , is_feasible_fitness_individual(&individual, &info)); } println!("]"); println!("Population len: {}", population.len()) } */

        // Stagnation
        let curr_fitness = population.last().unwrap().fitness;
        if best_fitness > curr_fitness {
            stagnation_counter = 0;
            best_fitness = curr_fitness;
            // Only save good solutions
            if curr_fitness < info.benchmark * 1.05 {
                save_individual(&population, &config);
            }
        } else if stagnation_counter > (config.n_stagnations) {
            stagnation_counter = 0;
            scramble_population(&mut population, &info, &config);
            fitness_population(&mut population, &info, &config);
            best_fitness = get_best_fitness_population(&population);
        } else {
            stagnation_counter += 1;
        }
        let mut elitism_members = get_elitism_members(&population, &config);
        population.drain(0..config.n_elitism as usize);

        let parent_indices: Vec<usize> = parent_selection(&mut population, &config);

        let mut children_population = population_crossover(&mut population, &parent_indices, &info, &config);

        mutate_population(&mut children_population, &config, &info);

        fitness_population(&mut children_population, &info, &config);

        survivor_selection(&mut population, &parent_indices, &mut children_population, &config);

        population.append(&mut elitism_members);

    }
}