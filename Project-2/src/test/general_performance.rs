use std::time::Instant;
use cpu_time::ProcessTime;
use crate::genetic::initialize_population::init_population;
use crate::genetic::survivor_selection::{similarity_hashmap, similarity};
use crate::test::mutation::get_initial;

#[test]
fn similarity_perf_test() {
    let (info, config, population) = get_initial();
    let individual1 = population[0].clone();
    let individual2 = population[1].clone();

    let start = ProcessTime::now();
    let similarity1 = similarity_hashmap(&individual1, &individual2, 2);
    let time_elapsed_hash = start.elapsed().as_micros();
    println!("Time used for hashmap method: {}", time_elapsed_hash);

    let start = ProcessTime::now();
    let similarity2 = similarity(&individual1, &individual2, 2);
    let time_elapsed = start.elapsed().as_micros();
    println!("Time used for vec method: {}", time_elapsed);

    if time_elapsed_hash < time_elapsed {
        println!("Hashmap is faster");
    } else {
        println!("Vec is faster");
    }

    assert_eq!(similarity1, similarity2);
}