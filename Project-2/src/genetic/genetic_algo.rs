use crate::genetic::initialize_population::{clustering, start_time};

use crate::structs::io;
use crate::structs::config::Config;
use crate::structs::nurse::Nurse;
use crate::structs::config::InitialPopType;

use crate::util::plot::plot_points;

pub(crate) fn start(train_path: &str, conf_path: &str) {
    let info = io::read_from_json(train_path).unwrap();
    let config = Config::new(conf_path);

    println!("Initializing population!");
    let mut population: Vec<Nurse> = match config.initial_pop_function {
        InitialPopType::Clustering => clustering(&info),
        InitialPopType::StartTime => start_time(&info)
    };
    println!("Done initializing population!");

    // Mutate
    // Evaluate

    for _ in 0..config.n_generations {

    }
}

