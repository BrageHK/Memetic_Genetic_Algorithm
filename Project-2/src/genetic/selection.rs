use crate::structs::nurse::Individual;

pub fn population_crossover(population: &Vec<Individual>) -> Vec<f32> {
    let sum: f32 = population.iter().map(|individual| individual.fitness).sum();

    let probabilities = population.iter().map(|individual| individual.fitness/sum).collect::<Vec<f32>>();

    Vec::new()
}