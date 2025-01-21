use crate::bitstring::BitArray;
use crate::config::{load_config, Config, PW, get_data};

pub struct SGA {
    pub population: Vec<(BitArray, u32)>,
    config: Config,
    data: Vec<PW>
}

impl SGA {
    pub fn new() -> Self {
        let config = load_config("config.yaml").unwrap();
        let mut population  = vec![(BitArray::new_random(), 0); config.population_size as usize];

        let data = get_data("knapPI_12_500_1000_82.csv");

        SGA{population, config, data}
    }


    pub fn fitness(bitarray: BitArray) -> i32 {
        let mut fitness: i32= 0;
        let mut weight: u32= 0;

        for i in 0..Self.data.len() {
            let x = bitarray.get_bit(i);
            if x {
                let data_point = Self.data.get(i).unwrap();
                fitness += (data_point.p * (weight <= Self.config.knapsack_capacity) as u32) as i32;
                weight += data_point.w;
            }
        }

        if weight > Self.config.knapsack_capacity {
            fitness -= (weight - (Self.config.knapsack_capacity as f32 * Self.config.overweight_penalty_factor) as u32) as i32;
        }
        fitness
    }

    pub fn evaluate() {
        Self.population = Self.population.iter().map(Self::fitness).for_each(|(fitness, weight)| {});
    }
}