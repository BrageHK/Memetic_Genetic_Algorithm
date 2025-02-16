use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Nurse {
    pub capacity: i32,
    pub route: Vec<i32>
}

impl Nurse {
    pub fn new() -> Self {
        Nurse{capacity: 0, route: Vec::new()}
    }
}