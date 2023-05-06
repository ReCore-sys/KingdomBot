use crate::types::factions::Faction;
use crate::types::map::Tile;
use crate::types::users::User;
use crate::{db, Error};

pub async fn clean_users() -> Result<(), Error> {
    let users = db::users::get_all().await?;
    let mut cleaned_users: Vec<User> = Vec::new();
    for user in users {
        let mut cleaned_user = user.clone();
        cleaned_user.permissions.dedup();
        cleaned_user
            .permissions
            .sort_by(|a, b| a.to_string().cmp(&b.to_string()));
        cleaned_user.faction = cleaned_user.faction.to_uppercase();
        cleaned_users.push(cleaned_user);
    }
    db::users::set_many(cleaned_users).await?;
    Ok(())
}

pub async fn clean_tiles() -> Result<(), Error> {
    let tiles = db::tiles::get_all().await?;
    let mut cleaned_tiles: Vec<Tile> = Vec::new();
    let mut to_delete: Vec<Tile> = Vec::new();

    for tile in tiles {
        let mut cleaned_tile = tile.clone();
        cleaned_tile.faction = cleaned_tile.faction.to_uppercase();
        let mut to_delete_tile = false;
        if cleaned_tile.faction == "" {
            to_delete_tile = true;
        }
        if cleaned_tile.buildings.len() == 0 && cleaned_tile.units.len() == 0 {
            to_delete_tile = true;
        }
        if !cleaned_tile.occupied {
            to_delete_tile = true;
        }
        if to_delete_tile {
            to_delete.push(cleaned_tile.clone());
        } else {
            cleaned_tiles.push(cleaned_tile.clone());
        }
    }
    db::tiles::set_many(cleaned_tiles).await?;
    let conn = db::get_db().await?;
    for tile in to_delete {
        db::tiles::internal_delete_tile(&conn, tile.x, tile.y).await?;
    }
    Ok(())
}

pub async fn clean_factions() -> Result<(), Error> {
    let factions = db::factions::get_all().await?;
    let mut cleaned_factions: Vec<Faction> = Vec::new();
    let mut to_delete: Vec<Faction> = Vec::new();

    for faction in factions {
        let mut cleaned_faction = faction.clone();
        cleaned_faction.tag = cleaned_faction.tag.to_uppercase();
        cleaned_faction.members.dedup();
        let mut to_delete_faction = false;
        if cleaned_faction.tag == "" {
            to_delete_faction = true;
        }
        if cleaned_faction.name == "" {
            to_delete_faction = true;
        }
        if cleaned_faction.members.len() == 0 {
            to_delete_faction = true;
        }
        if to_delete_faction {
            to_delete.push(cleaned_faction.clone());
        } else {
            cleaned_factions.push(cleaned_faction.clone());
        }
    }
    db::factions::set_many(cleaned_factions).await?;
    let conn = db::get_db().await?;
    for faction in to_delete {
        db::factions::internal_delete_faction(&conn, faction.tag).await?;
    }
    Ok(())
}