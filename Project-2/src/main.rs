extern crate core;

mod genetic;
mod structs;
mod util;
mod test;

use genetic::genetic_algo;
use crate::util::plot::{plot_best_individual, plot_points};

fn main() {
    //plot_best_individual();
    genetic_algo::start("config/config.yaml");
}
