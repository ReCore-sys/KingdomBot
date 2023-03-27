use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub(crate) struct Faction {
    pub(crate) name: String,
    // The name of the faction
    pub(crate) tag: String,
    // The 4 character tag of the faction
    pub(crate) description: String,
    // The description of the faction
    pub(crate) members: Vec<String>,
    // The UUIDs of the faction members
    pub(crate) leader: String,
    // The leader of the faction
    pub(crate) capital_x: i32,
    // The x coordinate of the faction capital
    pub(crate) capital_y: i32,
    // The y coordinate of the faction capital
    pub(crate) money: f64,
    // The amount of money the faction has
    pub(crate) money_per_tick: f64, // The amount of money the faction gets per second
}