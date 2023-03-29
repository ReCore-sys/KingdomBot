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
    pub(crate) production: Production,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub(crate) struct Production {
    pub(crate) money: f64,
    // The amount of money the faction has
    pub(crate) money_per_second: f64,
    // The amount of money the faction gets per second
    pub(crate) population: i32,
    // How many people live across the faction
    pub(crate) population_per_second: f64,
    // How many people the faction gets per second. We only actually increase this once an hour
    pub(crate) food: f64,
    // How much food the faction has
    pub(crate) food_per_second: f64,
    // How much food the faction gets per second
    pub(crate) happiness: f64,
    // How happy the faction is on a 1-100 scale
}