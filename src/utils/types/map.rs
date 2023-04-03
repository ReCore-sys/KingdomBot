use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Unit {
    pub(crate) soldier: i64,
    pub(crate) ranger: i64,
    pub(crate) cavalry: i64,
    pub(crate) knight: i64,
    pub(crate) public: i64,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Building {
    pub(crate) building_type: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TileOccupant {
    pub(crate) building: Building,
    pub(crate) units: Unit,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Tile {
    pub(crate) occupied: bool,
    pub(crate) faction: String,
    pub(crate) occupant: TileOccupant,
    pub(crate) x: i32,
    pub(crate) y: i32,
}
