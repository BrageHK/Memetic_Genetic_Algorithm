use crate::structs::config::{Config, ParentSelectionFN};
use crate::structs::nurse::Individual;

use rand::rng;
use rand::rngs::ThreadRng;
use rand::distr::Distribution;
use rand::distr::weighted::WeightedIndex;

type RankingFN = fn(&Vec<Individual>, &Config) -> Vec<usize>;

/// Do parent selection based on config and get parent indices
pub fn parent_selection(population: &mut Vec<Individual>, config: &Config) -> Vec<usize> {
    let parent_fn: RankingFN = match config.parent_selection_fn {
        ParentSelectionFN::LinearRanking => linear_ranking
    };

    let mut parent_indices = parent_fn(population, &config);
    if (parent_indices.len() & 1) != 0 {
        parent_indices.remove(0);
    }

    parent_indices
}

pub fn linear_rank_probability(mu: usize, s: f32, i: usize) -> f32 {
    ((2. - s)/mu as f32) + ((2 * i) as f32 * (s - 1.)) / (mu * (mu - 1)) as f32
}

/// Return indices of the best individuals_9.
/// Population must be sorted
pub fn linear_ranking(population: &Vec<Individual>, config: &Config) -> Vec<usize> {
    let mu = population.len();
    let mut rng: ThreadRng = rng();
    let n_parents = ((population.len() - config.n_elitism as usize) as f32 * config.n_parents_scaling) as usize;

    let probabilities: Vec<f32> = population
        .iter()
        .enumerate()
        .map(|individual| linear_rank_probability(mu, config.s, individual.0))
        .collect::<Vec<f32>>();

    let dist = WeightedIndex::new(&probabilities).unwrap();

    let mut indices = Vec::new();
    for _ in 0..n_parents {
        indices.push(dist.sample(&mut rng))
    }

    indices
}

pub fn probabilistic_ranking(population: &Vec<Individual>, config: &Config) -> Vec<usize> {
    let mut rng: ThreadRng = rng();
    let n_parents: usize = (population.len() - config.n_elitism as usize) * 3;
    let sum: f32 = population
        .iter()
        .map(|individual| individual.fitness)
        .sum();

    let probabilities = population
        .iter()
        .map(|individual| individual.fitness/sum)
        .collect::<Vec<f32>>();

    let mut dist = WeightedIndex::new(&probabilities).unwrap();

    let mut indices = Vec::new();
    for _ in 0..n_parents {
        indices.push(dist.sample(&mut rng))
    }

    indices
}