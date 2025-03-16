extern crate core;

mod genetic;
mod structs;
mod util;
mod test;

use mpi::request::WaitGuard;
use mpi::traits::*;

use genetic::genetic_algo;

use crate::structs::config::Config;
use crate::util::plot::plot_best_individual;

fn main() {
    let universe = mpi::initialize().unwrap();
    let world = universe.world();
    let size = world.size();
    let rank = world.rank();

    let config = Config::new("./config/config.yaml");
    if config.print_and_graph {
        println!("Starting plotting");
        plot_best_individual();
        println!("Finished plotting");
    }
    genetic_algo::init(config);
}
