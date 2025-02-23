use std::fs::File;
use std::io::Write;
use ordered_float::OrderedFloat;
use crate::structs::nurse::Individual;

pub fn save_individual(population: &Vec<Individual>) {
    let mut individual: Vec<Vec<i32>> = Vec::new();
    let best_individual = population
        .iter()
        .min_by_key(|i| OrderedFloat(i.fitness))
        .unwrap()
        .clone();
    for nurse in &best_individual.nurses {
        let incremented_route = nurse.route.iter().map(|&num| num + 1).collect();
        individual.push(incremented_route);
    }
    let mut file = File::create("individuals/".to_string() + &*best_individual.fitness.to_string()).unwrap();
    file.write_all(format!("{:?}",&individual).as_bytes()).unwrap();
}