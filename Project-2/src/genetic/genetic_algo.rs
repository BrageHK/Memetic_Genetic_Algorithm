use crate::genetic::evaluate::{fitness_nurse, fitness_population};
use crate::genetic::initialize_population::{clustering, start_time};
use crate::genetic::mutation::{mutate_nurse, mutate_population};
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
    fitness_population(&mut population, &info, &config);
    println!("{}", population[0].fitness);

    for _ in 0..config.n_generations {
        // Crossover
        mutate_population(&mut population, &config);
        // Selection
    }
}