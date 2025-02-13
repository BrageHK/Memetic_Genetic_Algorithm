use serde::Deserialize;

use serde_json::from_reader;

use std::error::Error;
use std::io::{BufReader, Read};
use std::fs::File;

#[derive(Deserialize, Debug)]
pub struct Info {
    pub instance_name: String,
    pub nbr_nurses: u32,
    pub capacity_nurse: u32,
    pub benchmark: f32,
}

pub struct Depot {
    pub return_time: u32,
    pub x_coord: u32,
    pub y_coord: u32
}

pub struct Patient {
    pub x_coord: u32,
    pub y_coord: u32,
    pub demand: u32,
    pub start_time: u32,
    pub end_time: u32,
    pub care_time: u32
}

pub struct TravelTimes {
    pub times: [i32; 100]
}

pub fn read_from_json(path: &str) -> Result<Info, Box<dyn Error>>{
    let f = File::open(path)?;
    let reader = BufReader::new(f);
    let info = from_reader(reader)?;
    Ok(info)
}