use mongodb::options::ClientOptions;
use mongodb::{Client, Database};

use crate::db::tiles::get_all_by_faction;
use crate::types::buildings::Building;
use crate::types::factions::Production;
use crate::{db, Error};

#[path = "database/cleaners.rs"]
pub mod cleaners;
#[path = "database/factions.rs"]
pub(crate) mod factions;
#[path = "database/tiles.rs"]
#[allow(dead_code)]
pub(crate) mod tiles;
#[path = "database/users.rs"]
pub(crate) mod users;

/// Gets a MongoDB database connection
///
///
/// # Returns
/// ```Database```: The database connection
///

pub async fn get_db() -> Result<Database, mongodb::error::Error> {
    let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await?;
    client_options.connect_timeout = Some(std::time::Duration::from_secs(1));
    client_options.app_name = Some("data".to_string());
    let client = Client::with_options(client_options)?;
    Ok(client.database("data"))
}

pub async fn build_production() -> Result<(), Error> {
    let db = get_db().await?;
    cleaners::clean_factions().await?;
    cleaners::clean_tiles().await?;
    cleaners::clean_users().await?;
    let factions = factions::get_all().await?;
    for mut faction in factions {
        let all_tiles = get_all_by_faction(faction.tag.clone()).await?;
        let faction_production = faction.production.clone();
        let mut production = Production {
            money: faction_production.money,
            money_per_second: 0.0,
            population: faction_production.population,
            population_per_second: 0.0,
            food: faction_production.food,
            food_per_second: 0.0,
            wood: faction_production.wood,
            wood_per_second: 0.0,
            metal: faction_production.metal,
            metal_per_second: 0.0,
            population_cap: 0,
            happiness: faction_production.happiness,
        };
        for tile in all_tiles {
            for (building, amount) in tile.buildings {
                use Building::*;
                match building {
                    Farm => production.food_per_second += (amount as f64) * 0.25,
                    Mill => production.wood_per_second += (amount as f64) * 0.2,
                    Blacksmith => production.metal_per_second += (amount as f64) * 0.3,
                    Capital => {
                        production.population_per_second += 0.15;
                        production.population_cap += 100;
                        production.food_per_second += 0.1;
                        production.wood_per_second += 0.05;
                        production.metal_per_second += 0.05;
                    }
                    House => {
                        production.population_per_second += 0.07 * amount as f64;
                        production.population_cap += 5 * amount as i64;
                    }

                    Hut => {
                        production.population_per_second += 0.05 * amount as f64;
                        production.population_cap += 3 * amount as i64;
                    }
                    _ => {}
                }
            }
        }
        faction.production = production;
        db::factions::save_faction(faction).await?;
    }
    Ok(())
}