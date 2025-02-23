use std::collections::HashMap;
use std::time::Instant;
use ordered_float::OrderedFloat;
use crate::genetic::evaluate::{fitness_population, get_best_fitness_population, is_feasible_fitness_individual, get_best_solution_population};
use crate::genetic::initialize_population::feasible_population_init;
use crate::genetic::mutation::mutate_population;
use crate::genetic::selection::population_crossover;
use crate::structs::io;
use crate::structs::config::Config;
use crate::structs::nurse::{Individual, Nurse};
use crate::structs::config::InitialPopType;

pub(crate) fn start(conf_path: &str) {
    let config = Config::new(conf_path);
    let info = io::read_from_json(&config).unwrap();

    for i in 0..3 {
        println!("{} {}", &info.patients[i].start_time, &info.patients[i].end_time);
    }

    let mut population: Vec<Individual> = match config.initial_pop_function {
        //InitialPopType::Clustering => clustering(&info),
        InitialPopType::StartTime => {
            feasible_population_init(&info, &config)
        },
        _ => panic!("Invalid initial pop function!")
    };
    // Population is not that diverse with the initial_population, therefore mutate them.
    // Evaluate
    let mut fitness_hashmap: HashMap<Vec<Nurse>, f32> = HashMap::new();

    fitness_population(&mut population, &info, &config, &mut fitness_hashmap, 0);


    let now = Instant::now();
    let mut stagnation_counter: i32 = 0;
    let mut best_fitness: f32 = f32::INFINITY;
    let mut best_solution: Vec<Vec<i32>> = Vec::new();
    let mut local_fitness_optima: f32 = f32::INFINITY;
    for i in 0..config.n_generations {

        population.sort_by(|p1, p2| p2.fitness.total_cmp(&p1.fitness));
        if i % 100 == 0 {
            println!("Generation: {} Best curr fitness: {}", i, get_best_fitness_population(&population));
            println!("Best overall fitness {}", best_fitness);
            println!("local optima fitness {}", best_fitness);
            println!("num individuals: {}", &population.len());
            println!("Is feasible_solution? {:?}", is_feasible_fitness_individual(&population.iter().min_by_key(|i| OrderedFloat(i.fitness)).unwrap(), &info));
            //for nurse in &population.iter().min_by_key(|i| OrderedFloat(i.fitness)).unwrap().nurses {
                //print!("{} ", nurse.route.len());
            //}
            //println!();
        }

        if i % 1000 == 0 {
            let mut individual: Vec<Vec<i32>> = Vec::new();
            let best_individual = population.iter().min_by_key(|i| OrderedFloat(i.fitness)).unwrap().clone();
            for nurse in &best_individual.nurses {
                let incremented_route = nurse.route.iter().map(|&num| num + 1).collect();
                individual.push(incremented_route);
            }
            println!("{:?}", &individual);
            println!("{:?}", &best_solution);
        }
        let mut elitism_members = population[population.len()-config.n_elitism as usize..].to_vec();

        let new_population = population_crossover(&mut population, &info, &config, i, stagnation_counter);

        population = new_population;

        population.append(&mut elitism_members);

        //TODO: fix mutation
        //mutate_population(&mut population, &config);

        fitness_population(&mut population, &info, &config, &mut fitness_hashmap, i);

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
    let elapsed = now.elapsed();
    println!("Time used: {} ms", elapsed.as_millis());
}