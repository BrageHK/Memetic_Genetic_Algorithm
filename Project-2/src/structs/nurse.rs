use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, Eq, Hash, PartialEq)]
pub struct Nurse {
    pub capacity: i32,
    pub route: Vec<i32>
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Individual {
    pub nurses: Vec<Nurse>,
    pub fitness: f32
}

impl Nurse {
    pub fn new() -> Self {
        Nurse{capacity: 0, route: Vec::new()}
    }
}