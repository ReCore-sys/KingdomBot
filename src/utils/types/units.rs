use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Hash, Eq, PartialEq, Clone)]
pub(crate) enum Unit {
    Citizen,
    Soldier,
    Cavalry,
    Ranger,
    Knight,
    Scout,
}

#[derive(Debug, Serialize, Deserialize, Hash, Eq, PartialEq, Default)]
pub(crate) struct UnitData {
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) space: i32,
    pub(crate) beats: Vec<Unit>,
}

impl Unit {
    pub fn data(&self) -> UnitData {
        use Unit::*;
        let (name, description, space, beats) = match self {
            Citizen => ("Citizen", "A normal citizen of the faction", 1, vec![]),
            Soldier => ("Soldier", "A regular soldier", 1, vec![Citizen]),
            Cavalry => ("Cavalry", "A fast moving cavalry unit", 1, vec![Soldier]),
            Ranger => ("Ranger", "A long range unit", 1, vec![Soldier, Cavalry]),
            Knight => (
                "Knight",
                "A heavy hitting knight",
                1,
                vec![Cavalry, Soldier],
            ),
            Scout => ("Scout", "A fast moving scout", 1, vec![Citizen]),
        };
        UnitData {
            name: name.to_string(),
            description: description.to_string(),
            space,
            beats: beats.to_vec(),
        }
    }
}