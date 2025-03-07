extern crate core;

mod genetic;
mod structs;
mod util;
mod test;

use genetic::genetic_algo;
use crate::util::plot::plot_best_individual;

fn main() {
    println!("Starting plotting");
    plot_best_individual();
    println!("Finished plotting");
    genetic_algo::start("config/config.yaml");
}
