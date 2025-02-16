extern crate core;

mod genetic;
mod structs;
mod util;

use genetic::genetic_algo;

fn main() {
    genetic_algo::start("train/train_0.json", "config.yaml");
}
