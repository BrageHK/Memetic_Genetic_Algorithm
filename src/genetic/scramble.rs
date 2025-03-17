use crate::genetic::initialize_population::init_population;
use crate::structs::config::{Config, ScrambleFN};
use crate::structs::io::Info;
use crate::structs::nurse::Individual;

type ScrambleFNType = fn(&mut Vec<Individual>, &Info, &Config);

pub fn scramble_population(population: &mut Vec<Individual>, info: &Info, config: &Config) {
    println!("Scramble population:");
    let scramble: ScrambleFNType = match config.scramble_fn {
        ScrambleFN::Delete => delete,
        ScrambleFN::Keep => delete,
    };
    scramble(population, &info, &config);
}

fn delete(individuals: &mut Vec<Individual>, info: &Info, config: &Config) {
    let new_population = init_population(&info, &config);
    *individuals = new_population;
}
fn keep(individuals: &mut Vec<Individual>, info: &Info, config: &Config) {
    let best_individual = individuals.last().unwrap().clone();
    let new_population = init_population(&info, &config);
    *individuals = new_population;
    individuals.remove(individuals.len()-1);
    individuals.push(best_individual);
}