use rand::Rng;
use rand::rngs::ThreadRng;
use crate::genetic::evaluate::fitness_nurse;
use crate::structs::config::Config;
use crate::structs::io::Info;
use crate::structs::nurse::Nurse;

pub fn destroy_and_repair(individual: &mut Vec<Nurse>, rng: &mut ThreadRng, info: &Info, config: &Config) {
    // Should nurse with more than 6 patients
    let mut nurse_idx = 0;
    let mut found = false;
    let mut len = 0;
    for _ in 0..3 {
        nurse_idx = rng.random_range(0..individual.len());
        if individual[nurse_idx].route.len() >= 6 {
            len = individual[nurse_idx].route.len();
            found = true;
            break;
        }
    }
    if !found {
        return;
    }

    let n_to_move = rng.random_range(2..len);

    let patients_to_move: Vec<i32> = individual[nurse_idx].route.drain(len-n_to_move..).collect();

    // Optimal insert
    let mut lowest_fitness = f32::INFINITY;
    let (mut best_nurse_i, mut best_patient_i) = (0,0);

    for nurse_i in 0..individual.len() {
        for patient_pos in 0..individual[nurse_i].route.len() {
            individual[nurse_i].route.splice(patient_pos..patient_pos, patients_to_move.clone().into_iter());
            let after_fitness_nurse = fitness_nurse(&individual[nurse_i], &info, &config);
            individual[nurse_i].route.drain(patient_pos..patient_pos+patients_to_move.len());
            if after_fitness_nurse < lowest_fitness {
                lowest_fitness = after_fitness_nurse;
                best_nurse_i = nurse_i;
                best_patient_i = patient_pos;
            }
        }
    }

    individual[best_nurse_i].route.splice(best_patient_i..best_patient_i, patients_to_move);
}