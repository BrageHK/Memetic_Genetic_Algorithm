use cpu_time::ProcessTime;

use crate::genetic::evaluate::{fitness_population, get_best_fitness_population, get_best_solution_population};
use crate::genetic::initialize_population::init_population;
use crate::genetic::mutation::mutate_population;
use crate::genetic::parent_selection::parent_selection;
use crate::genetic::crossover::population_crossover;
use crate::genetic::elitism::get_elitism_members;
use crate::genetic::scramble::scramble_population;
use crate::genetic::survivor_selection::survivor_selection;
use crate::structs::io;
use crate::structs::config::Config;
use crate::util::save_individual::save_individual;

pub(crate) fn start(conf_path: &str) {
    let config = Config::new(conf_path);
    let info = io::read_from_json(&config).unwrap();

    let mut population = init_population(&info, &config);

    fitness_population(&mut population, &info, &config);

    let mut stagnation_counter: i32 = 0;
    let mut best_fitness: f32 = f32::INFINITY;

    let start = ProcessTime::now();

    for i in 0..config.n_generations {
        population.sort_by(|p1, p2| p2.fitness.total_cmp(&p1.fitness));

        if i % 100 == 0 {
            let fitnesses: Vec<f32> = population.iter().map(|x| x.fitness).collect::<Vec<f32>>();
            let last_fitnesses = &fitnesses[fitnesses.len()-5..];
            println!("nGenerations: {} Best fitnesses: {:?} Execution_time {:?}", i, last_fitnesses, &start.elapsed());
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

        survivor_selection(&mut population, &parent_indices, &children_population, &config);

        population.append(&mut elitism_members);
    }
}