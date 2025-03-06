#[cfg(test)]
use std::collections::HashMap;
#[cfg(test)]
use std::sync::{Arc, Mutex};
#[cfg(test)]
use std::thread;
#[cfg(test)]
use crate::genetic::evaluate::{fitness_population, get_best_fitness_population, get_best_solution_population};
#[cfg(test)]
use crate::genetic::initialize_population::init_population;
#[cfg(test)]
use crate::genetic::mutation::mutate_population;
#[cfg(test)]
use crate::genetic::parent_selection::parent_selection;
#[cfg(test)]
use crate::genetic::crossover::population_crossover;
#[cfg(test)]
use crate::genetic::survivor_selection::survivor_selection;
#[cfg(test)]
use crate::structs::io;
#[cfg(test)]
use crate::structs::config::Config;
#[cfg(test)]
use crate::structs::io::Info;
#[cfg(test)]
use crate::structs::nurse::{Individual, Nurse};

#[cfg(test)]
struct Island {
    population: Vec<Individual>,
    fitness_hashmap: HashMap<Vec<Nurse>, f32>,
    best_fitness: f32,
    best_solution: Vec<Vec<i32>>,
    stagnation_counter: i32,
}

#[cfg(test)]
impl Island {
    fn new(info: &Info, config: &Config) -> Self {
        let population = init_population(info, config);
        let fitness_hashmap = HashMap::new();
        Island {
            population,
            fitness_hashmap,
            best_fitness: f32::INFINITY,
            best_solution: Vec::new(),
            stagnation_counter: 0,
        }
    }

    fn evolve(&mut self, info: &Info, config: &Config) {
        fitness_population(&mut self.population, info, config);

        for _ in 0..config.n_generations {
            self.population.sort_by(|p1, p2| p2.fitness.total_cmp(&p1.fitness));

            let mut elitism_members = self.population[self.population.len()-config.n_elitism as usize..].to_vec();
            self.population.drain(0..config.n_elitism as usize);

            // Stagnation
            let curr_fitness = get_best_fitness_population(&self.population);
            if self.best_fitness > curr_fitness {
                self.stagnation_counter = 0;
                self.best_fitness = curr_fitness;
                if curr_fitness < 870. {
                    self.best_solution = get_best_solution_population(&self.population);
                }
            } else if self.stagnation_counter > config.n_stagnations {
                self.stagnation_counter = 0;
                self.scramble_population(info, config);
                fitness_population(&mut self.population, info, config);
                self.best_fitness = get_best_fitness_population(&self.population);
            } else {
                self.stagnation_counter += 1;
            }

            let parent_indices: Vec<usize> = parent_selection(&mut self.population, config);

            let mut children_population = population_crossover(&mut self.population, &parent_indices, info, config);

            mutate_population(&mut children_population, config, info);

            fitness_population(&mut children_population, info, config);

            survivor_selection(&mut self.population, &parent_indices, &children_population, config);

            self.population.append(&mut elitism_members);
        }
    }

    fn scramble_population(&mut self, info: &Info, config: &Config) {
        println!("Stagnated! Scrambling!");
        let mut new_population = init_population(info, config);
        self.population = new_population;
    }
}

#[cfg(test)]
pub(crate) fn start(conf_path: &str) {
    let config = Arc::new(Config::new(conf_path));
    let info = Arc::new(io::read_from_json(&config).unwrap());

    let num_islands = 4; // Number of islands
    let islands: Vec<Arc<Mutex<Island>>> = (0..num_islands)
        .map(|_| Arc::new(Mutex::new(Island::new(&info, &config))))
        .collect();

    let mut handles = vec![];

    for island in islands {
        let config = Arc::clone(&config);
        let info = Arc::clone(&info);
        let handle = thread::spawn(move || {
            let mut island = island.lock().unwrap();
            island.evolve(&info, &config);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // After all islands have evolved, you can implement migration logic here
    // For example, exchange the best individuals_9 between islands periodically
}