use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub(crate) enum Building {
    Farm,
    Mine,
    Capital,
    House,
    Hut,
}

#[derive(Debug, Serialize, Deserialize, Hash, Eq, PartialEq, Default)]
pub(crate) struct BuildingData {
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) cost: i32,
    pub(crate) space: i32,
}

impl Building {
    pub fn data(&self) -> BuildingData {
        use Building::*;
        let (name, description, cost, space) = match self {
            Farm => (
                "Farm".to_string(),
                "A farm that produces food".to_string(),
                100,
                1,
            ),
            Mine => (
                "Mine".to_string(),
                "A mine that produces ore".to_string(),
                100,
                4,
            ),
            Capital => (
                "Capital".to_string(),
                "The capital of the faction".to_string(),
                0,
                100,
            ),

            House => (
                "House".to_string(),
                "A house that people can live in".to_string(),
                100,
                10,
            ),
            Hut => (
                "Hut".to_string(),
                "A hut that people can live in".to_string(),
                50,
                5,
            ),
        };
        BuildingData {
            name,
            description,
            cost,
            space,
        }
    }
}