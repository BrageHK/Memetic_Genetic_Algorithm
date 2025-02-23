use std::collections::HashMap;
use std::time::Instant;
use crate::genetic::evaluate::{fitness_population, get_best_fitness_population, is_feasible_fitness_individual, get_best_solution_population};
use crate::genetic::initialize_population::init_population;
use crate::genetic::mutation::mutate_population;
use crate::genetic::parent_selection::parent_selection;
use crate::genetic::crossover::population_crossover;
use crate::genetic::survivor_selection::survivor_selection;
use crate::structs::io;
use crate::structs::config::Config;
use crate::structs::nurse::Nurse;
use crate::util::save_individual::save_individual;

pub(crate) fn start(conf_path: &str) {
    let config = Config::new(conf_path);
    let info = io::read_from_json(&config).unwrap();

    let mut population = init_population(&info, &config);

    let mut fitness_hashmap: HashMap<Vec<Nurse>, f32> = HashMap::new();

    fitness_population(&mut population, &info, &config, &mut fitness_hashmap);

    let mut stagnation_counter: i32 = 0;
    let mut best_fitness: f32 = f32::INFINITY;
    let mut best_solution: Vec<Vec<i32>> = Vec::new();
    let mut local_fitness_optima: f32 = f32::INFINITY;

    let start = Instant::now();

    for i in 0..config.n_generations {

        population.sort_by(|p1, p2| p2.fitness.total_cmp(&p1.fitness));

        if i % 10 == 0 { println!("nGenerations: {} Best fitness: {} Execution_time {:?}", i, &population.last().unwrap().fitness, &start.elapsed()); }
        if i % 100 == 0 && population.last().unwrap().fitness < 877. { save_individual(&population) }

        let mut elitism_members = population[population.len()-config.n_elitism as usize..].to_vec();
        population.drain(0..config.n_elitism as usize);

        let parent_indices: Vec<usize> = parent_selection(&mut population, &config);

        let mut children_population = population_crossover(&mut population, &parent_indices, &info, &config);

        mutate_population(&mut children_population, &config, &info);

        fitness_population(&mut children_population, &info, &config, &mut fitness_hashmap);

        let start_survivor = Instant::now();
        survivor_selection(&mut population, &children_population, &config);
        println!("Survivor time: {:?}", &start_survivor.elapsed());

        population.append(&mut elitism_members);

        // Stagnation
        /*
        let best_fitness_now = get_best_fitness_population(&population);
        if best_fitness_now < local_fitness_optima || stagnation_counter > (config.n_stagnations) {
            stagnation_counter = 0;
            if best_fitness_now < best_fitness {
                best_solution = get_best_solution_population(&population);
                best_fitness = get_best_fitness_population(&population);
            }
            local_fitness_optima = best_fitness_now;
        } else {
            stagnation_counter += 1;
        }

         */
    }
}