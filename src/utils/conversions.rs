use crate::types::map::Tile;

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

use crate::commands::faction::FactionModal;
use crate::types::factions::Faction;
use crate::types::users::User;
use poise::serenity_prelude::User as SerenityUser;

pub(crate) async fn convert_user(user: &SerenityUser) -> User {
    User {
        uuid: user.id.to_string(),
        username: user.name.clone(),
        discriminator: user.discriminator.to_string(),
        faction: "".to_string(),
    }
}

pub(crate) async fn modal_to_faction(modal: FactionModal) -> Faction {
    let mut faction = Faction::default();
    faction.name = modal.faction_name;
    faction.description = modal.faction_description;
    faction.tag = modal.faction_tag.to_uppercase();
    faction
}