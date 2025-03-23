use crate::genetic::evaluate::{duration_demand_nurse, fitness_print_nurse};
use crate::structs::io::Info;
use crate::structs::nurse::Nurse;

pub fn print_best_solution(individual: Vec<Vec<i32>>, info: &Info) {
    println!("Nurse capacity: {}", info.capacity_nurse);
    println!("Depot return time: {}", info.depot.return_time);
    println!("Nurse idx\tDur\tDemand\tRoute");
    for (nurse_idx, nurse) in individual.iter().enumerate() {
        let (capacity, duration) = duration_demand_nurse(&Nurse {capacity: 0, route: nurse.clone()}, &info);
        print!("Nurse {} \t{:.2}\t{}\tD(0)", nurse_idx, duration, capacity);
        fitness_print_nurse(&nurse, &info);
        print!(" -> D({:.2})\n", duration);
    }
}
