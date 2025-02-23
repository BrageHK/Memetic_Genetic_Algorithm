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
use crate::structs::io::Info;
use crate::structs::nurse::{Individual, Nurse};
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

    let start = Instant::now();

    for i in 0..config.n_generations {
        population.sort_by(|p1, p2| p2.fitness.total_cmp(&p1.fitness));


        if i % 10 == 0 { println!("nGenerations: {} Best fitness: {} Execution_time {:?}", i, &population.last().unwrap().fitness, &start.elapsed()); }
        if i % 100 == 0 && population.last().unwrap().fitness < 877. { save_individual(&population) }

        let mut elitism_members = population[population.len()-config.n_elitism as usize..].to_vec();
        population.drain(0..config.n_elitism as usize);
        // Stagnation
        let curr_fitness = get_best_fitness_population(&population);
        if best_fitness > curr_fitness {
            stagnation_counter = 0;
            best_fitness = curr_fitness;
            if curr_fitness < 877. {
                best_solution = get_best_solution_population(&population);
            }
        } else if stagnation_counter > (config.n_stagnations) {
            stagnation_counter = 0;
            scramble_population(&mut population, &info, &config);
            fitness_population(&mut population, &info, &config, &mut fitness_hashmap);
        } else {
            stagnation_counter += 1;
        }

        let parent_indices: Vec<usize> = parent_selection(&mut population, &config);

        let mut children_population = population_crossover(&mut population, &parent_indices, &info, &config);

        mutate_population(&mut children_population, &config, &info);

        fitness_population(&mut children_population, &info, &config, &mut fitness_hashmap);

        //let start_survivor = Instant::now();
        survivor_selection(&mut population, &children_population, &config);
        //println!("Survivor time: {:?}", &start_survivor.elapsed());

        population.append(&mut elitism_members);
    }
}

fn scramble_population(population: &mut Vec<Individual>, info: &Info, config: &Config) {
    println!("Stagnated! Scrambling!");
    let mut new_population = init_population(&info, &config);
    new_population.drain(0..config.n_elitism as usize);
    *population = new_population;
}