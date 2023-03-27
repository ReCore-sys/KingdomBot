use poise::serenity_prelude::AttachmentType;
use tokio::fs::File;

use db::tiles;

use crate::conversions::{bytes_to_string, split_tiles};
use crate::db::tiles::{blank_tile, invert_y};
use crate::map::Tile;
use crate::{db, Context, Error};

// The parent command. Doesn't really need to do anything.
#[poise::command(slash_command, prefix_command, subcommands("position", "dev"))]
pub(crate) async fn map(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}

// The user-facing command. This is the one that gets called when the user types /map position
// it calls on the create_reply function to get the image, but ignores the message
#[poise::command(slash_command, prefix_command)]
pub(crate) async fn position(
    ctx: Context<'_>,
    #[description = "X coordinate of the centre tile"] x: i32,
    #[description = "Y coordinate of the centre tile"] y: i32,
) -> Result<(), Error> {
    ctx.defer().await?;
    let (file, _) = create_reply(x, y).await?;
    let attachment = AttachmentType::File {
        file: &file,
        filename: "map.png".to_string(),
    };

    ctx.send(|b| b.attachment(attachment)).await?;
    Ok(())
}

// Same as the position command, but this one sends the message as well
#[poise::command(slash_command, prefix_command, track_edits)]
pub(crate) async fn dev(
    ctx: Context<'_>,
    #[description = "X coordinate of the centre tile"] x: i32,
    #[description = "Y coordinate of the centre tile"] y: i32,
) -> Result<(), Error> {
    ctx.defer().await?;
    let (file, dev_message) = create_reply(x, y).await?;
    let mut send_message = dev_message;
    // Since the file was created in the create_reply function, get the size here
    // We could get it in the function, but that means we have to open the file twice and it's
    // not really worth it
    send_message.push_str(&format!(
        "\nFile size: {}",
        bytes_to_string(file.metadata().await?.len())
    ));
    let attachment = AttachmentType::File {
        file: &file,
        filename: "map.png".to_string(),
    };

    ctx.send(|b| b.attachment(attachment).content(send_message))
        .await?;
    Ok(())
}

// This is the function that actually does all the work. Creates the image and the status message
async fn create_reply(x: i32, y: i32) -> Result<(File, String), Error> {
    let mut dev_message = String::new();
    let mut tiles: Vec<Vec<Tile>> = Vec::new();
    let offset_base = crate::image::VIEW_DISTANCE / 2; // How far to go in each direction
    let offset_x_min = x - offset_base;
    let offset_x_max = x + offset_base;
    let offset_y_min = y - offset_base;
    let offset_y_max = y + offset_base;
    let x_range = (offset_x_min, offset_x_max);
    let y_range = (offset_y_min, offset_y_max);
    let tiles_exist = tiles::all_exist(x_range, y_range).await;
    dev_message.push_str("Saved tiles: ");
    // Check if all tiles exist
    if tiles_exist {
        dev_message.push_str("100%");
        // If they do, get them from the database
        let flat_tiles = tiles::get_many(x_range, y_range).await;
        println!("{} tiles", flat_tiles.len());
        tiles = split_tiles(flat_tiles, crate::image::VIEW_DISTANCE);
    } else {
        // First see if any tiles exist
        if !tiles::any_exist(x_range, y_range).await {
            // If they don't, it's easier to just make a load of blank tiles
            dev_message.push_str("0%");
            let empty_tiles = tiles::blank_tile_range(x_range, y_range).await;
            tiles = split_tiles(empty_tiles, crate::image::VIEW_DISTANCE);
        } else {
            // If only have some of the tiles, we need to get the ones that exist and
            // generate the rest
            let max_tiles = crate::image::VIEW_DISTANCE * crate::image::VIEW_DISTANCE;
            let mut tiles_exist = 0;
            let database = db::get_db().await;
            for x in offset_x_min..offset_x_max {
                let mut tile_row: Vec<Tile> = Vec::new();
                for y in offset_y_min..offset_y_max {
                    let current_tile: Tile;
                    if !tiles::internal_check_tile(&database, x, y).await {
                        current_tile = blank_tile(x, y).await;
                    } else {
                        tiles_exist += 1;
                        current_tile = tiles::internal_get_tile(&database, x, y).await;
                    }
                    tile_row.push(current_tile);
                }
                tiles.push(tile_row);
            }
            // If VIEW_DISTANCE is 10, then max_tiles will be 100 so it's pretty easy
            // tho if the VIEW_DISTANCE is changed, this will still work
            let percentage = (tiles_exist as f32 / max_tiles as f32) * 100.0;
            dev_message.push_str(&format!("{}%", percentage));
        }
    }
    // None of the percentage messages include a newline, so add it here
    dev_message.push_str("\n");
    // Record the time it takes to generate the map
    let mut start = std::time::Instant::now();
    tiles = invert_y(tiles).await;
    let image = crate::image::draw_map(&tiles).await;
    dev_message.push_str(&format!(
        "Map generated in {}ms",
        start.elapsed().as_millis()
    ));
    // Reset the timer and save the image
    start = std::time::Instant::now();
    image.save("map.png")?;
    dev_message.push_str(&format!(
        "\nImage saved in {}ms",
        start.elapsed().as_millis()
    ));
    dev_message.push_str(&format!(
        "\nImage dimensions: {}x{}",
        image.width(),
        image.height()
    ));
    let file = File::open("map.png").await?;
    Ok((file, dev_message))
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