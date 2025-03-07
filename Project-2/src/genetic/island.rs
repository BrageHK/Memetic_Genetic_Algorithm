use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::available_parallelism;
use crate::genetic::genetic_algo::start;
use crate::structs::io;
use crate::structs::config::Config;

pub(crate) fn start_islands(config: Config) {
    //let _shared_individuals = Arc::new(Mutex::new(Vec::new()));
    let num_islands: usize = available_parallelism().unwrap().get();
    let mut handles = vec![];

    println!("Creating {} islands!", num_islands);
    for _ in 0..num_islands {
        let handle = thread::spawn(move || {
            start(Config::new("./config/config.yaml"));
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}