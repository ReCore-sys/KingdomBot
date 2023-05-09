use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub(crate) struct Faction {
    #[serde(default)]
    pub(crate) name: String,
    // The name of the faction
    #[serde(default)]
    pub(crate) tag: String,
    // The 4 character tag of the faction
    #[serde(default)]
    pub(crate) description: String,
    // The description of the faction
    #[serde(default)]
    pub(crate) members: Vec<String>,
    // The UUIDs of the faction members
    #[serde(default)]
    pub(crate) leader: String,
    // The leader of the faction
    #[serde(default)]
    pub(crate) capital_x: i32,
    // The x coordinate of the faction capital
    #[serde(default)]
    pub(crate) capital_y: i32,
    // The y coordinate of the faction capital
    #[serde(default)]
    pub(crate) production: Production,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Copy)]
pub(crate) struct Production {
    #[serde(default)]
    pub(crate) last_updated: u64,
    // The last time the production was updated
    #[serde(default)]
    pub(crate) money: f32,
    // The amount of money the faction has
    #[serde(default)]
    pub(crate) money_per_second: f64,
    // The amount of money the faction gets per second
    #[serde(default)]
    pub(crate) population: f64,
    // How many people live across the faction
    #[serde(default)]
    pub(crate) population_per_second: f64,
    // How many people the faction gets per second. We only actually increase this once an hour
    #[serde(default)]
    pub(crate) food: f32,
    // How much food the faction has
    #[serde(default)]
    pub(crate) food_per_second: f64,
    // How much food the faction gets per second
    #[serde(default)]
    pub(crate) wood: f32,
    // How much wood the faction has
    #[serde(default)]
    pub(crate) wood_per_second: f64,
    // How much wood the faction gets per second
    #[serde(default)]
    pub(crate) metal: f32,
    // How much metal the faction has
    #[serde(default)]
    pub(crate) metal_per_second: f64,
    // How much metal the faction gets per second
    #[serde(default)]
    pub(crate) happiness: f64,
    // How happy the faction is on a 1-100 scale
    #[serde(default)]
    pub(crate) population_cap: i64,
    // The maximum population the faction can have
}