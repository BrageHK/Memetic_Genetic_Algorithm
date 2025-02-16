use crate::bitstring::BitArray;
use crate::config::{load_config, Config, PW, get_data};

use rand::{random, Rng};
use std::cmp::{max, Reverse};
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{self, Write};
use serde_json;

#[derive(Clone)]
pub struct Individual {
    pub(crate) bit_array: BitArray,
    pub(crate) fitness: i32
}

#[derive(Serialize)]
struct GraphData {
    pub min: i32,
    pub max: i32,
    pub mean: i32
}
pub struct SGA {
    pub population: Vec<Individual>,
    config: Config,
    data: Vec<PW>,
    graph_data: Vec<GraphData>
}

impl SGA {
    pub fn new() -> Self {
        let config = load_config("config.yaml").unwrap();
        let mut population  = vec![
            Individual{bit_array: BitArray::new_random(), fitness: 0};
            config.population_size as usize
        ];

        let data = get_data("data/knapPI_12_500_1000_82.csv");

        SGA{population, config, data, graph_data: Vec::new()}
    }

    pub fn run(&mut self) {
        self.evaluate();
        for i in 0..self.config.n_generations {
            let mut parents = Vec::new();
            for j in 0..self.config.population_size {
                parents.push(self.select_parent().unwrap());
            }

            let mut children: Vec<Individual> = Vec::new();
            for _ in 0..self.config.population_size/2 - self.config.n_elitism {
                let parent1 = parents.pop().unwrap();
                let parent2 = parents.pop().unwrap();
                let (mut child1, mut child2) = self.crossover(&parent1, &parent2);
                self.mutate(&mut child1);
                self.mutate(&mut child2);
                children.push(child1);
                children.push(child2);
            }
            children.append(&mut self.get_best_individuals().clone());
            self.population = children;
            self.evaluate();
            println!("Max fitness: {}, generation {}", self.get_max_fitness(), i);
            self.append_graph_data();
        }
    }

    fn append_graph_data(&mut self) {
        let max = self.get_max_fitness();
        let mean = self.get_mean_fitness();
        let min = self.get_min_fitness();
        self.graph_data.push(GraphData{max, mean, min})
    }

    fn get_best_individuals(&mut self) -> Vec<Individual> {
        self.population.sort_by_key(|x| Reverse(x.fitness));
        self.population.clone().into_iter().take(self.config.n_elitism as usize).collect()
    }

    pub fn get_max_fitness(&self) -> i32 {
        self.population.iter().max_by_key(|x| x.fitness).unwrap().fitness
    }

    pub fn get_min_fitness(&self) -> i32 {
        self.population.iter().min_by_key(|x| x.fitness).unwrap().fitness
    }

    pub fn get_mean_fitness(&self) -> i32 {
        self.total_fitness() / self.population.len() as i32
    }

    pub fn fitness(bitarray: &BitArray, config: &Config, data: &Vec<PW>) -> i32 {
        let mut fitness: i32= 0;
        let mut weight: i32= 0;

        for i in 0..data.len() {
            let x = bitarray.get_bit(i);
            if x {
                let data_point = data.get(i).unwrap();
                if weight <= config.knapsack_capacity {
                    fitness += data_point.p * (weight <= config.knapsack_capacity) as i32;
                } else {
                    fitness -= data_point.p * (weight <= config.knapsack_capacity) as i32;
                }
                weight += data_point.w;
            }
        }

        max(fitness, 0)
    }

   pub fn evaluate(&mut self) {
        for individual in &mut self.population {
            individual.fitness = SGA::fitness(&individual.bit_array, &self.config, &self.data);
        }
    }

    fn total_fitness(&self) -> i32 {
        let mut sum = 0;
        for individual in &self.population {
            sum += individual.fitness;
        }
        sum
    }

    fn mutate(&self, individual: &mut Individual) {
        for bit in 0..500 {
            if random::<f32>() <= self.config.mutation_rate {
                individual.bit_array.flip_bit(bit);
            }
        }
    }

    fn select_parent(&self) -> Option<&Individual> {
        let random_value: f64 = random();
        let total_fitness: i32 = self.total_fitness();
        let mut cumulative_fitness: f64 = 0.0;

        for individual in &self.population {
            cumulative_fitness += (individual.fitness as f64)/total_fitness as f64;
            if random_value <= cumulative_fitness {
                return Some(individual);
            }
        }

        None
    }

    fn crossover(&self, parent1: &Individual, parent2: &Individual) -> (Individual, Individual) {
        if random::<f32>() >= self.config.crossover_rate {
            return (parent1.clone(), parent2.clone())
        }

        let crossover_point = rand::thread_rng().gen_range(0..500);
        let mut child1_bit_arr = BitArray::new();
        for i in 0..crossover_point {
            let other_bit = parent1.bit_array.get_bit(i);
            child1_bit_arr.set_bit(i, other_bit);
        }
        for i in crossover_point..500 {
            let other_bit = parent2.bit_array.get_bit(i);
            child1_bit_arr.set_bit(i, other_bit);
        }
        let mut child2_bit_arr = BitArray::new();
        for i in 0..crossover_point {
            let other_bit = parent2.bit_array.get_bit(i);
            child2_bit_arr.set_bit(i, other_bit);
        }
        for i in crossover_point..500 {
            let other_bit = parent1.bit_array.get_bit(i);
            child2_bit_arr.set_bit(i, other_bit);
        }
        let child1 = Individual{bit_array: child1_bit_arr, fitness: 0};
        let child2 = Individual{bit_array: child2_bit_arr, fitness: 0};
        (child1, child2)
    }

    pub fn write_to_file(&self, filename: &str) {
        let mut file = File::create(filename).unwrap();
        for i in 0..3 {
            for datapoint in &self.graph_data {
                let data = match i {
                    0 => datapoint.max,
                    1 => datapoint.min,
                    2 => datapoint.mean,
                    _ => -1
                };
                write!(file, "{},", data).expect("TODO: panic message");
            }
            writeln!(file).expect("TODO: panic message");
        }

    }
}