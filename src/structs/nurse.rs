use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

#[derive(Deserialize, Serialize, Debug, Clone, Eq, Hash, PartialEq)]
pub struct Nurse {
    pub capacity: i32,
    pub route: Vec<i32>
}

impl Nurse {
    pub fn new() -> Self {
        Nurse{capacity: 0, route: Vec::new()}
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Individual {
    pub nurses: Vec<Nurse>,
    pub fitness: f32
}

impl Hash for Individual {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.nurses.hash(state);
    }
}

impl Eq for Individual { }

impl PartialEq for Individual {
    fn eq(&self, other: &Self) -> bool {
        self.nurses == other.nurses
    }
}
