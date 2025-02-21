use std::collections::HashMap;
use std::time::Instant;
use crate::genetic::evaluate::{fitness_nurse, fitness_population, get_best_fitness_population};
use crate::genetic::initialize_population::{clustering, start_time};
use crate::genetic::mutation::{mutate_nurse, mutate_population};
use crate::genetic::selection::population_crossover;
use crate::structs::io;
use crate::structs::config::Config;
use crate::structs::nurse::{Individual, Nurse};
use crate::structs::config::InitialPopType;

use crate::util::plot::plot_points;

pub(crate) fn start(train_path: &str, conf_path: &str) {
    let info = io::read_from_json(train_path).unwrap();
    let config = Config::new(conf_path);

    println!("Initializing population!");
    let mut population: Vec<Individual> = match config.initial_pop_function {
        //InitialPopType::Clustering => clustering(&info),
        InitialPopType::StartTime => {
            vec![Individual{nurses: start_time(&info), fitness: 0.0}; config.population_size as usize]
        },
        _ => panic!("Invalid initial pop function!")
    };
    println!("Done initializing population!");
    // Population is not that diverse with the initial_population, therefore mutate them.
    // Mutate
    mutate_population(&mut population, &config);
    // Evaluate
    let mut fitness_hashmap: HashMap<Vec<Nurse>, f32> = HashMap::new();

    fitness_population(&mut population, &info, &config, &mut fitness_hashmap);


    let now = Instant::now();
    for i in 0..config.n_generations {
        println!("Generation: {}", i);

        population.sort_by(|p1, p2| p2.fitness.total_cmp(&p1.fitness));
        let mut elitism_members = population[population.len()-config.n_elitism as usize..].to_vec();

        let new_population = population_crossover(&mut population, &info, &config);

        population = new_population;

        mutate_population(&mut population, &config);

        population.append(&mut elitism_members);

        fitness_population(&mut population, &info, &config, &mut fitness_hashmap);
        println!("Best fitness: {}", get_best_fitness_population(&population));
    }
    let elapsed = now.elapsed();
    println!("Time used: {} ms", elapsed.as_millis());
}