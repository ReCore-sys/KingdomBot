use poise::serenity_prelude::User as SerenityUser;

use crate::commands::faction::FactionModal;
use crate::types::buildings::Building;
use crate::types::factions::Faction;
use crate::types::map::Tile;
use crate::types::units::Unit;
use crate::types::users::User;

pub fn bytes_to_string(bytes: u64) -> String {
    let mut bytes = bytes as f64;
    let suffixes = ["B", "KB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];
    for suffix in suffixes.iter() {
        if bytes < 1024.0 {
            return format!("{:.2} {}", bytes, suffix);
        }
        bytes /= 1024.0;
    }
    format!("{} {}", bytes, "YB")
}

pub fn split_tiles(tiles: Vec<Tile>, size: i32) -> Vec<Vec<Tile>> {
    let mut tiles = tiles;
    let mut tile_rows: Vec<Vec<Tile>> = Vec::new();
    for _ in 0..size {
        let mut tile_row: Vec<Tile> = Vec::new();
        for _ in 0..size {
            tile_row.push(tiles.remove(0));
        }
        tile_rows.push(tile_row);
    }
    tile_rows
}

pub(crate) async fn convert_user(user: &SerenityUser) -> User {
    User {
        uuid: user.id.to_string(),
        username: user.name.clone(),
        discriminator: user.discriminator.to_string(),
        faction: "".to_string(),
        permissions: Vec::new(),
    }
}

pub(crate) async fn modal_to_faction(modal: &FactionModal) -> Faction {
    let mut faction = Faction::default();
    faction.name = modal.faction_name.clone();
    faction.description = modal.faction_description.clone();
    faction.tag = modal.faction_tag.to_uppercase();
    faction
}

pub(crate) async fn string_to_unit(string: &String) -> Result<Unit, bool> {
    match string.as_str() {
        "citizen" => Ok(Unit::Citizen),
        "soldier" => Ok(Unit::Soldier),
        "cavalry" => Ok(Unit::Cavalry),
        "ranger" => Ok(Unit::Ranger),
        "knight" => Ok(Unit::Knight),
        "scout" => Ok(Unit::Scout),
        _ => Err(false),
    }
}

/// Converts a string to a building
///
/// # Arguments
///
/// * `string`: The name of the building to get
///
/// returns: Result<Building, bool>
pub(crate) async fn string_to_building(string: &String) -> Result<Building, bool> {
    match string.to_lowercase().as_str() {
        "farm" => Ok(Building::Farm),
        "blacksmith" => Ok(Building::Blacksmith),
        "mill" => Ok(Building::Mill),
        "barracks" => Ok(Building::Barracks),
        "hut" => Ok(Building::Hut),
        "house" => Ok(Building::House),
        "capital" => Ok(Building::Capital),
        _ => Err(false),
    }
}