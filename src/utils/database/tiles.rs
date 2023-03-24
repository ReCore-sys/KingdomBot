use futures::stream::TryStreamExt;
use mongodb::bson::doc;
use mongodb::options::FindOptions;
use mongodb::Database;

use crate::db;
use crate::map::{Building, Tile, TileOccupant, Unit};

/// Gets a tile. Uses a pre-existing database connection
///
/// # Arguments
///
/// * `db` - A database connection
/// * `x` - The x value of the tile
/// * `y` - The y value of the tile
///
/// # Returns
/// ```Tile```: The tile at the specified coordinates
///

pub async fn internal_get_tile(db: &Database, x: i32, y: i32) -> Tile {
    let collection = db.collection::<Tile>("tiles");
    let filter = doc! {"x": x, "y": y};
    let options = FindOptions::builder().limit(1).build();
    let mut cursor = collection.find(filter, options).await.unwrap();
    let mut tile = blank_tile(x, y).await;
    let result = cursor.try_next().await;
    match result {
        Ok(document) => match document {
            Some(d) => {
                tile = d;
            }
            None => {
                println!("No document found");
            }
        },
        Err(e) => {
            println!("Error: {}", e);
        }
    }

    tile
}

/// Sets a tile. Uses a pre-existing database connection
///
/// # Arguments
///
/// * `db` - A database connection
/// * `tile` - The tile to set. Since the x and y values are set in the tile, you don't need to provide them
///
/// # Returns
/// ```Result```: The result of the operation
///

pub(crate) async fn internal_set_tile(
    db: &Database,
    tile: Tile,
) -> Result<(), mongodb::error::Error> {
    let collection = db.collection::<Tile>("tiles");
    let filter = doc! {"x": tile.x, "y": tile.y};
    if !internal_check_tile(&db, tile.x, tile.y).await {
        let result = collection.insert_one(tile, None).await;
        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    } else {
        let result = collection.find_one_and_replace(filter, tile, None).await;
        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
}

/// Checks if a tile exists. Uses a pre-existing database connection
///
/// # Arguments
///
/// * `db` - A database connection
/// * `x` - The x value of the tile
/// * `y` - The y value of the tile
///
/// # Returns
/// ```bool```: Whether or not the tile exists
///

pub(crate) async fn internal_check_tile(db: &Database, x: i32, y: i32) -> bool {
    let collection = db.collection::<Tile>("tiles");
    let filter = doc! {"x": x, "y": y};
    let options = FindOptions::builder().limit(1).build();
    let cursor = collection.find(filter, options).await.unwrap();
    let all: Vec<Tile> = cursor.try_collect().await.unwrap();
    if all.len() == 0 {
        false
    } else {
        true
    }
}

/// A more accessible version of internal_check_tile. Creates its own database connection
///
/// # Arguments
///
/// * `x` - The x value of the tile
/// * `y` - The y value of the tile
///
/// # Returns
/// ```bool```: Whether or not the tile exists
///

pub async fn check_tile(x: i32, y: i32) -> bool {
    let db = db::get_db().await;
    internal_check_tile(&db, x, y).await
}

/// Checks if a range of tiles exist. Creates its own database connection
///
/// # Arguments
///
/// * `x_range` - The minimum and maximum x values of the tiles
/// * `y_range` - The minimum and maximum y values of the tiles
///
/// # Returns
/// ```Vec<bool>```: A vector of booleans, where each boolean represents whether or not the tile exists
///

pub async fn check_many(x_range: (i32, i32), y_range: (i32, i32)) -> Vec<bool> {
    let db = db::get_db().await;
    let mut tiles = Vec::new();
    for x in x_range.0..x_range.1 {
        for y in y_range.0..y_range.1 {
            tiles.push(internal_check_tile(&db, x, y).await);
        }
    }
    tiles
}

/// Given a range of tiles, checks if all of them exist. Creates its own database connection
///
/// # Arguments
///
/// * `x_range` - The minimum and maximum x values of the tiles
/// * `y_range` - The minimum and maximum y values of the tiles
///
/// # Returns
/// ```bool```: Whether or not all of the tiles exist
///

pub async fn all_exist(x_range: (i32, i32), y_range: (i32, i32)) -> bool {
    let db = db::get_db().await;
    for x in x_range.0..x_range.1 {
        for y in y_range.0..y_range.1 {
            if !internal_check_tile(&db, x, y).await {
                return false;
            }
        }
    }
    true
}

/// Checks if any of the tiles in a range exist. Creates its own database connection
///
/// # Arguments
///
/// * `x_range` - The minimum and maximum x values of the tiles
/// * `y_range` - The minimum and maximum y values of the tiles
///
/// # Returns
/// ```bool```: Whether or not any of the tiles exist
///
pub async fn any_exist(x_range: (i32, i32), y_range: (i32, i32)) -> bool {
    let db = db::get_db().await;
    let filter = doc! {"x": {"$gte": x_range.0, "$lte": x_range.1}, "y": {"$gte": y_range.0, "$lte": y_range.1}};
    let options = FindOptions::builder().limit(1).build();
    let collection = db.collection::<Tile>("tiles");
    let cursor = collection.find(filter, options).await.unwrap();
    let all: Vec<Tile> = cursor.try_collect().await.unwrap();
    if all.len() == 0 {
        false
    } else {
        true
    }
}

/// Gets a tile. Creates its own database connection
///
/// # Arguments
///
/// * `x` - The x value of the tile
/// * `y` - The y value of the tile
///
/// # Returns
/// ```Tile```: The tile at the specified coordinates
///

pub async fn get_tile(x: i32, y: i32) -> Tile {
    let db = db::get_db().await;
    internal_get_tile(&db, x, y).await
}

/// Gets a range of tiles. Creates its own database connection
///
/// # Arguments
///
/// * `x_range` - The minimum and maximum x values of the tiles
/// * `y_range` - The minimum and maximum y values of the tiles
///
/// # Returns
/// ```Vec<Tile>```: A list of tiles in the specified range
///

pub async fn get_many(x_range: (i32, i32), y_range: (i32, i32)) -> Vec<Tile> {
    let db = db::get_db().await;
    let filter = doc! {"x": {"$gte": x_range.0, "$lte": x_range.1}, "y": {"$gte": y_range.0, "$lte": y_range.1}};
    let options = FindOptions::builder().build();
    let cursor = db
        .collection::<Tile>("tiles")
        .find(filter, options)
        .await
        .unwrap();
    let all: Vec<Tile> = cursor.try_collect().await.unwrap();
    all
}

/// A more accessible version of internal_get_tile. Creates its own database connection
///
/// # Arguments
///
/// * `tile` - The tile to store
///
/// # Returns
/// ```Result<(), mongodb::error::Error>```: The result of the operation
///

pub async fn set_tile(tile: Tile) -> Result<(), mongodb::error::Error> {
    let db = db::get_db().await;
    internal_set_tile(&db, tile).await
}

/// Sets many tiles. Creates its own database connection
///
/// # Arguments
///
/// * `tiles` - The tiles to store
///
/// # Returns
/// ```Result<(), mongodb::error::Error>```: The result of the operation
///

pub async fn set_many(tiles: Vec<Tile>) -> Result<(), mongodb::error::Error> {
    let db = db::get_db().await;
    for tile in tiles {
        let t = internal_set_tile(&db, tile).await;
        if t.is_err() {
            return Err(t.err().unwrap());
        }
    }
    Ok(())
}

/// Creates a blank tile
///
///
/// # Returns
/// ```Tile```: A blank tile
///

pub async fn blank_tile(x: i32, y: i32) -> Tile {
    let mut t = Tile::default();
    t.x = x;
    t.y = y;
    t
}

/// Generates a list of blank tiles between the specified ranges
///
/// # Arguments
///
/// * `x_range` - The range of x values
/// * `y_range` - The range of y values
///
/// # Returns
/// ```Vec<Tile>```: A list of blank tiles with the specified ranges
///

pub async fn blank_tile_range(x_range: (i32, i32), y_range: (i32, i32)) -> Vec<Tile> {
    let mut tiles = Vec::new();
    for x in x_range.0..x_range.1 {
        for y in y_range.0..y_range.1 {
            let t = blank_tile(x, y).await;
            tiles.push(t);
        }
    }
    tiles
}