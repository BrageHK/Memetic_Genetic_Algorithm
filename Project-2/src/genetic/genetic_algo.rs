use std::collections::HashMap;
use ordered_float::OrderedFloat;
use crate::genetic::evaluate::{fitness_population, get_best_fitness_population, is_feasible_fitness_individual, get_best_solution_population};
use crate::genetic::initialize_population::init_population;
use crate::genetic::mutation::mutate_population;
use crate::genetic::parent_selection::parent_selection;
use crate::genetic::crossover::population_crossover;
use crate::structs::io;
use crate::structs::config::Config;
use crate::structs::nurse::Nurse;
use crate::util::save_individual::save_individual;

pub(crate) fn start(conf_path: &str) {
    let config = Config::new(conf_path);
    let info = io::read_from_json(&config).unwrap();

    let mut population = init_population(&info, &config);

    let mut fitness_hashmap: HashMap<Vec<Nurse>, f32> = HashMap::new();

    fitness_population(&mut population, &info, &config, &mut fitness_hashmap, 0);

    let mut stagnation_counter: i32 = 0;
    let mut best_fitness: f32 = f32::INFINITY;
    let mut best_solution: Vec<Vec<i32>> = Vec::new();
    let mut local_fitness_optima: f32 = f32::INFINITY;

    for i in 0..config.n_generations {
        if i % 100 == 0 { println!("nGenerations: {} Curr fitness: {} Best fitness: {} Legal: {:?}", i, get_best_fitness_population(&population), best_fitness, is_feasible_fitness_individual(&population.iter().min_by_key(|i| OrderedFloat(i.fitness)).unwrap(), &info)); }
        if i % 1000 == 0 && get_best_fitness_population(&population) < 877. { save_individual(&population) }

        // Sort by fitness (best are last)
        population.sort_by(|p1, p2| p2.fitness.total_cmp(&p1.fitness));
        // Get elitism individuals
        let mut elitism_members = population[population.len()-config.n_elitism as usize..].to_vec();
        // Parent selection
        let parent_indices: Vec<usize> = parent_selection(&mut population, &info, &config);
        // Crossover
        let mut children_population = population_crossover(&mut population, &parent_indices, &info, &config, i, stagnation_counter);
        // Mutate children
        mutate_population(&mut children_population, &config, &info);
        // Evaluate population
        fitness_population(&mut children_population, &info, &config, &mut fitness_hashmap, i);
        // Survivor selection
        population.append(&mut elitism_members);

        // Stagnation
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
    }
}