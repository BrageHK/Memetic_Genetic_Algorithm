use rand::prelude::IndexedRandom;
use crate::structs::io::{Info, Patient};
use crate::structs::nurse::{Individual, Nurse};

use rand::Rng;
use crate::genetic::evaluate::is_feasible_fitness_nurse;
use crate::structs::config::Config;
use crate::util::plot::plot_points;

#[derive(Debug)]
pub(crate) struct Point {
    pub(crate) x: f32,
    pub(crate) y: f32
}

/// Return the index of the furthest patient
fn furthest_point(points: &Vec<Point>, patients: &Vec<&Patient>) -> usize {
    let mut max_distance = 0.0;
    let mut max_index = 0;
    for (i, patient) in patients.iter().enumerate() {
        let distance = points
            .iter()
            .map(|point| ((patient.x_coord - point.x).powf(2.0) + (patient.y_coord - point.y).powf(2.0)).sqrt())
            .max_by(|a, b| a.total_cmp(b))
            .unwrap();
        if distance > max_distance {
            max_distance = distance;
            max_index = i;
        }
    }
    max_index
}

pub fn clustering(info: &Info) -> Vec<Nurse> {
    let mut rng = rand::rng();

    let nurses: Vec<Nurse> = Vec::new();
    let mut edge_points: Vec<Point> = Vec::new();

    let first_patient_idx = rng.random_range(0..info.patients.len());
    let first_patient = info.patients.get(first_patient_idx).unwrap();

    let mut available_patients = info.patients.iter().clone().enumerate().collect::<Vec<(usize, &Patient)>>();
    available_patients.remove(first_patient_idx);

    edge_points.push(Point{x: first_patient.x_coord, y: first_patient.y_coord});

    // Create edge points
    for _ in 1..info.nbr_nurses {
        let idx = furthest_point(&edge_points, &available_patients.iter().map(|(_, p)| *p).collect());
        let patient = info.patients.get(idx).unwrap();
        available_patients.remove(idx);
        let point = Point{x: patient.x_coord, y: patient.y_coord};
        edge_points.push(point);
    }

    println!("Points: {:?}", &edge_points);

    plot_points(&edge_points);

    Vec::new()
}

pub fn start_time(info: &Info) -> Vec<Nurse> {
    let mut rng = rand::rng();
    let mut patients = info.patients.iter().clone().enumerate().collect::<Vec<(usize, &Patient)>>();
    patients.sort_by(|p1, p2| p1.1.start_time.partial_cmp(&p2.1.start_time).unwrap());
    println!("Patient_first_start_time: {:?}", &patients[0].1.start_time);
    println!("Patient_last_start_time: {:?}", &patients[patients.len() - 1].1.start_time);

    for _ in 0..0 {
        let idx = rng.random_range(0..patients.len()-1);
        patients.swap(idx, idx+1);
    }

    //println!("Patients {:?}", &patients);

    let mut nurses: Vec<Nurse> = vec![Nurse::new(); info.nbr_nurses as usize];

    for (i, &(patient_idx, p)) in patients.iter().enumerate() {
        let nurse_idx = i % info.nbr_nurses as usize;
        nurses[nurse_idx].route.push(patient_idx as i32);
        nurses[nurse_idx].capacity += p.demand as i32;
        if nurses[nurse_idx].capacity > info.capacity_nurse as i32 {
            panic!("Too big rip");
        }
    }

    nurses
}

pub fn feasible_population_init(info: &Info, config: &Config) -> Vec<Individual> {
    let mut population = Vec::new();
    for _ in 0..config.population_size {
        population.push(feasible_init_individual(&info, &config));
    }
    population
}

pub(crate) fn feasible_init_individual(info: &Info, config: &Config) -> Individual {
    let mut rng = rand::rng();
    let patients = info.patients
        .iter()
        .clone()
        .enumerate()
        .collect::<Vec<(usize, &Patient)>>();
    let mut nurses: Vec<Nurse> = vec![Nurse::new(); info.nbr_nurses as usize];

    for (patient_idx, _patient) in &patients {
        let mut n = 0;
        'outer: loop {
            let nurse_idx = rng.random_range(0..nurses.len());
            let route_idx =  if !nurses[nurse_idx].route.is_empty() {rng.random_range(0..nurses[nurse_idx].route.len())} else { 0 };
            nurses[nurse_idx].route.insert(route_idx, *patient_idx as i32);

            if is_feasible_fitness_nurse(&nurses[nurse_idx], &info) {
                break 'outer;
            }
            nurses[nurse_idx].route.remove(route_idx);

            n += 1;
            if n > 8050 {
                panic!("Bruh: {:?}, \ninfo: {:?}", &nurses, _patient);
            }
        }
    }
    Individual{nurses, fitness: -9999.}
}