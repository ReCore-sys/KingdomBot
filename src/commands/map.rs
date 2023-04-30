use std::io::Cursor;

use poise::serenity_prelude::AttachmentType;

use db::tiles;

use crate::conversions::{bytes_to_string, split_tiles};
use crate::db::tiles::{blank_tile, invert_y};
use crate::types::map::Tile;
use crate::{db, Context, Error};

// The parent command. Doesn't really need to do anything.
#[poise::command(
    slash_command,
    prefix_command,
    subcommands("position", "dev", "capital")
)]
pub(crate) async fn map(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}

// The user-facing command. This is the one that gets called when the user types /map position
// it calls on the create_reply function to get the image, but ignores the message
#[poise::command(
    slash_command,
    prefix_command,
    ephemeral,
    description_localized("en-US", "Get a map of the world around you")
)]
pub(crate) async fn position(
    ctx: Context<'_>,
    #[description = "X coordinate of the centre tile"] x: i32,
    #[description = "Y coordinate of the centre tile"] y: i32,
) -> Result<(), Error> {
    if !db::users::user_exists(ctx.author().id.to_string()).await? {
        ctx.say("You need to register first!\nUse `/register` to join!")
            .await?;
        return Ok(());
    }
    ctx.defer().await?;
    let tag = db::users::get_user(ctx.author().id.to_string())
        .await?
        .faction;
    let (data, _) = create_reply(x, y, tag).await?;

    let attachment = AttachmentType::Bytes {
        data: std::borrow::Cow::Owned(data.into_inner()),
        filename: "map.png".to_string(),
    };

    ctx.send(|b| b.attachment(attachment)).await?;
    Ok(())
}

// Same as the position command, but this one sends the message as well
#[poise::command(
    slash_command,
    prefix_command,
    track_edits,
    ephemeral,
    description_localized("en-US", "Same as /map position, but has extra debug info")
)]
pub(crate) async fn dev(
    ctx: Context<'_>,
    #[description = "X coordinate of the centre tile"] x: i32,
    #[description = "Y coordinate of the centre tile"] y: i32,
) -> Result<(), Error> {
    if !db::users::user_exists(ctx.author().id.to_string()).await? {
        ctx.say("You need to register first!\nUse `/register` to join!")
            .await?;
        return Ok(());
    }
    ctx.defer().await?;
    let tag = db::users::get_user(ctx.author().id.to_string())
        .await?
        .faction;
    let (data, dev_message) = create_reply(x, y, tag).await?;
    let mut send_message = dev_message;
    // Since the file was created in the create_reply function, get the size here
    // We could get it in the function, but that means we have to open the file twice and it's
    // not really worth it
    send_message.push_str(&format!(
        "\nFile size: {}",
        bytes_to_string(data.get_ref().len() as u64)
    ));
    let attachment = AttachmentType::Bytes {
        data: std::borrow::Cow::Owned(data.into_inner()),
        filename: "map.png".to_string(),
    };

    ctx.send(|b| b.attachment(attachment).content(send_message))
        .await?;
    Ok(())
}

#[poise::command(
    slash_command,
    prefix_command,
    track_edits,
    ephemeral,
    description_localized("en-US", "Automatically gets the map of your faction's capital")
)]
pub(crate) async fn capital(ctx: Context<'_>) -> Result<(), Error> {
    if !db::users::user_exists(ctx.author().id.to_string()).await? {
        ctx.say("You need to register first!\nUse `/register` to join!")
            .await?;
        return Ok(());
    }
    if db::users::get_user(ctx.author().id.to_string())
        .await?
        .faction
        == ""
    {
        ctx.say("You aren't in a faction yet!").await?;
        return Ok(());
    }
    ctx.defer().await?;
    let faction_tag = db::users::get_user(ctx.author().id.to_string())
        .await?
        .faction;
    let faction = db::factions::get_faction(faction_tag.clone()).await?;
    let (x, y) = (faction.capital_x, faction.capital_y);
    let (data, _) = create_reply(x, y, faction_tag).await?;
    let attachment = AttachmentType::Bytes {
        data: std::borrow::Cow::Owned(data.into_inner()),
        filename: "map.png".to_string(),
    };

    ctx.send(|b| b.attachment(attachment)).await?;
    Ok(())
}

// This is the function that actually does all the work. Creates the image and the status message
async fn create_reply(x: i32, y: i32, faction: String) -> Result<(Cursor<Vec<u8>>, String), Error> {
    let mut dev_message = String::new();
    let mut tiles: Vec<Vec<Tile>> = Vec::new();
    let offset_base = crate::image::VIEW_DISTANCE / 2; // How far to go in each direction
    let offset_x_min = x - offset_base;
    let offset_x_max = x + offset_base;
    let offset_y_min = y - offset_base;
    let offset_y_max = y + offset_base;
    let x_range = (offset_x_min, offset_x_max);
    let y_range = (offset_y_min, offset_y_max);
    let tiles_exist = tiles::all_exist(x_range, y_range).await?;
    dev_message.push_str("Saved tiles: ");
    // Check if all tiles exist
    if tiles_exist {
        dev_message.push_str("100%");
        // If they do, get them from the database
        let flat_tiles = tiles::get_many(x_range, y_range).await?;
        println!("{} tiles", flat_tiles.len());
        tiles = split_tiles(flat_tiles, crate::image::VIEW_DISTANCE);
    } else {
        // First see if any tiles exist
        if !tiles::any_exist(x_range, y_range).await? {
            // If they don't, it's easier to just make a load of blank tiles
            dev_message.push_str("0%");
            let empty_tiles = tiles::blank_tile_range(x_range, y_range).await;
            tiles = split_tiles(empty_tiles, crate::image::VIEW_DISTANCE);
        } else {
            // If only have some of the tiles, we need to get the ones that exist and
            // generate the rest
            let max_tiles = crate::image::VIEW_DISTANCE * crate::image::VIEW_DISTANCE;
            let mut tiles_exist = 0;
            let database = db::get_db().await?;
            for x in offset_x_min..offset_x_max {
                let mut tile_row: Vec<Tile> = Vec::new();
                for y in offset_y_min..offset_y_max {
                    let current_tile: Tile;
                    if !tiles::internal_check_tile(&database, x, y).await? {
                        current_tile = blank_tile(x, y).await;
                    } else {
                        tiles_exist += 1;
                        current_tile = tiles::internal_get_tile(&database, x, y).await?;
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
    tiles.remove(0);
    for row in tiles.iter_mut() {
        row.remove(row.len() - 1);
    }

    let image = crate::image::draw_map(&tiles, faction).await;
    dev_message.push_str(&format!(
        "Map generated in {}ms",
        start.elapsed().as_millis()
    ));
    // Reset the timer and save the image
    start = std::time::Instant::now();
    let mut cursor = Cursor::new(Vec::new());
    image
        .write_to(&mut cursor, image::ImageOutputFormat::Png)
        .unwrap();
    dev_message.push_str(&format!(
        "\nImage encoded in {}ms",
        start.elapsed().as_millis()
    ));
    dev_message.push_str(&format!(
        "\nImage dimensions: {}x{}",
        image.width(),
        image.height()
    ));
    Ok((cursor, dev_message))
}