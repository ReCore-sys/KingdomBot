use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Hash, Eq, PartialEq, Clone)]
pub(crate) enum Building {
    Farm,
    Mill,
    Blacksmith,
    Barracks,
    Capital,
    House,
    Hut,
}

#[derive(Debug, Serialize, Deserialize, Hash, Eq, PartialEq, Default, Clone)]
pub(crate) struct BuildingData {
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) cost: i32,
    pub(crate) wood: i32,
    pub(crate) metal: i32,
    pub(crate) space: i32,
}

impl Building {
    pub fn data(&self) -> BuildingData {
        use Building::*;
        // TODO: Balance this shit
        let (name, description, cost, wood, metal, space) = match self {
            Farm => (
                "Farm".to_string(),
                "A farm that produces food".to_string(),
                100,
                50,
                0,
                1,
            ),
            Mill => (
                "Mill".to_string(),
                "A mill that processes wood".to_string(),
                100,
                50,
                0,
                1,
            ),
            Blacksmith => (
                "Blacksmith".to_string(),
                "A blacksmith that forges metal".to_string(),
                100,
                50,
                0,
                5,
            ),
            Barracks => (
                "Barracks".to_string(),
                "Where your troops are trained".to_string(),
                300,
                800,
                500,
                10,
            ),
            Capital => (
                "Capital".to_string(),
                "The capital of the faction".to_string(),
                0,
                0,
                0,
                100,
            ),

            House => (
                "House".to_string(),
                "A house that people can live in".to_string(),
                150,
                300,
                50,
                10,
            ),
            Hut => (
                "Hut".to_string(),
                "A hut that people can live in".to_string(),
                50,
                100,
                50,
                5,
            ),
        };
        BuildingData {
            name,
            description,
            cost,
            wood,
            metal,
            space,
        }
    }
}