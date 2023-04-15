use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::types::buildings::Building;
use crate::types::units::Unit;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Tile {
    pub(crate) occupied: bool,
    pub(crate) faction: String,
    pub(crate) buildings: HashMap<Building, u32>,
    pub(crate) units: HashMap<Unit, u32>,
    pub(crate) x: i32,
    pub(crate) y: i32,
}