extern crate core;

mod genetic;
mod structs;
mod util;
mod test;

use genetic::genetic_algo;

fn main() {
    genetic_algo::start("config/config.yaml");
}
