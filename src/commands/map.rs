use db::tiles;
use poise::serenity_prelude::AttachmentType;
use tokio::fs::File;

use crate::db::tiles::{blank_tile, invert_y};
use crate::map::Tile;
use crate::{db, Context, Error};

#[poise::command(slash_command, prefix_command)]
pub(crate) async fn map(
    ctx: Context<'_>,
    #[description = "X coordinate of the centre tile"] x: i32,
    #[description = "Y coordinate of the centre tile"] y: i32,
) -> Result<(), Error> {
    ctx.defer().await?;
    let mut tiles: Vec<Vec<Tile>> = Vec::new();
    let offset_base = crate::image::VIEW_DISTANCE / 2; // How far to go in each direction
    let offset_x_min = x - offset_base;
    let offset_x_max = x + offset_base;
    let offset_y_min = y - offset_base;
    let offset_y_max = y + offset_base;
    let x_range = (offset_x_min, offset_x_max);
    let y_range = (offset_y_min, offset_y_max);
    let tiles_exist = tiles::all_exist(x_range, y_range).await; // Check if all tiles exist
    println!("Tiles exist: {}", tiles_exist);
    if tiles_exist {
        // If they do, get them from the database
        let flat_tiles = tiles::get_many(x_range, y_range).await;
        let mut tile_row: Vec<Tile> = Vec::new();
        let mut i = 0;
        println!("{} tiles", flat_tiles.len());
        for tile in flat_tiles {
            i += 1;
            tile_row.push(tile);
            if i % crate::image::VIEW_DISTANCE == 0 {
                tiles.push(tile_row);
                tile_row = Vec::new();
                i = 0;
            }
        }
    } else {
        if !tiles::any_exist(x_range, y_range).await {
            for x in offset_x_min..offset_x_max {
                let mut tile_row: Vec<Tile> = Vec::new();
                for y in offset_y_min..offset_y_max {
                    let current_tile = blank_tile(x, y).await;
                    tile_row.push(current_tile);
                }
                tiles.push(tile_row);
            }
        } else {
            // If they don't, get the tiles we can from the database and generate the rest
            let database = db::get_db().await;
            for x in offset_x_min..offset_x_max {
                let mut tile_row: Vec<Tile> = Vec::new();
                for y in offset_y_min..offset_y_max {
                    let current_tile: Tile;
                    if !tiles::internal_check_tile(&database, x, y).await {
                        current_tile = blank_tile(x, y).await;
                    } else {
                        current_tile = tiles::internal_get_tile(&database, x, y).await;
                    }
                    tile_row.push(current_tile);
                }
                tiles.push(tile_row);
            }
        }
    }
    let start = std::time::Instant::now();
    tiles = invert_y(tiles).await;
    let image = crate::image::draw_map(&tiles).await;
    println!("Map drawn in {}ms", start.elapsed().as_millis());
    image.save("map.png")?;
    let attachment = AttachmentType::File {
        file: &File::open("map.png").await?,
        filename: "map.png".to_string(),
    };

    ctx.send(|b| b.attachment(attachment)).await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub(crate) async fn create_tile(
    ctx: Context<'_>,
    #[description = "X coordinate of the tile"] x: i32,
    #[description = "Y coordinate of the tile"] y: i32,
) -> Result<(), Error> {
    if ctx
        .author_member()
        .await
        .unwrap()
        .permissions
        .unwrap()
        .administrator()
    {
        let tile = blank_tile(x, y).await;
        tiles::set_tile(tile).await.expect("Failed to set tile");
        ctx.say("Tile created").await?;
    } else {
        ctx.say("You do not have permission to use this command")
            .await?;
    }
    Ok(())
}