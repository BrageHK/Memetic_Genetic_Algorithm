#[cfg(test)]
use crate::genetic::initialize_population::init_population;
#[cfg(test)]
use crate::genetic::mutation::*;
#[cfg(test)]
use crate::structs::config::Config;
#[cfg(test)]
use crate::structs::io;
#[cfg(test)]
use crate::structs::io::Info;
#[cfg(test)]
use crate::structs::nurse::Individual;

#[cfg(test)]
pub fn get_initial() -> (Info, Config, Vec<Individual>) {
    let config = Config::new("config/config_test.yaml");
    let info = io::read_from_json(&config).unwrap();
    let population = init_population(&info, &config);
    (info, config, population)
}

#[cfg(test)]
fn legal(individual: &Individual) {
    let mut patients: Vec<i32> = Vec::new();
    for nurse in &individual.nurses {
        for p in &nurse.route {
            assert!(!patients.contains(p));
            patients.push(*p);
        }
    }
}


#[test]
fn test_heuristic_cluster_mutation() {
    let (info, config, population) = get_initial();
    let mut individual = population[0].clone();
    let mut rng = rand::rng();

    let individual_before = individual.clone();

    legal(&individual_before);

    for _ in 0..5 {
        heuristic_cluster_mutation(&mut individual.nurses, &mut rng, &info, &config);
        if individual_before != individual {
            break;
        }
    }
    assert_ne!(individual_before, individual);
    legal(&individual);
}