use crate::types::buildings::Building;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Unit {
    pub(crate) soldier: i64,
    pub(crate) ranger: i64,
    pub(crate) cavalry: i64,
    pub(crate) knight: i64,
    pub(crate) public: i64,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Tile {
    pub(crate) occupied: bool,
    pub(crate) faction: String,
    pub(crate) buildings: HashMap<Building, u32>,
    pub(crate) x: i32,
    pub(crate) y: i32,
}