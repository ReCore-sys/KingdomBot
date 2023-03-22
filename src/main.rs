use poise::serenity_prelude as serenity;
use poise::serenity_prelude::AttachmentType;
use tokio::fs::File;

use crate::map::{blank_tile, Tile};

#[path = "utils/config.rs"]
mod config;
#[path = "utils/db.rs"]
mod db;
#[path = "utils/image.rs"]
mod image;
#[path = "utils/map.rs"]
mod map;

struct Data {}

// User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command, prefix_command)]
async fn map(
    ctx: Context<'_>,
    #[description = "X coordinate of the centre tile"] x: i32,
    #[description = "Y coordinate of the centre tile"] y: i32,
) -> Result<(), Error> {
    ctx.defer().await?;
    let mut tiles: Vec<Vec<Tile>> = Vec::new();
    let offset_base = image::VIEW_DISTANCE / 2; // How far to go in each direction
    let offset_x_min = x - offset_base;
    let offset_x_max = x + offset_base;
    let offset_y_min = y - offset_base;
    let offset_y_max = y + offset_base;
    let x_range = (offset_x_min, offset_x_max);
    let y_range = (offset_y_min, offset_y_max);
    let tiles_exist = db::all_exist(x_range, y_range).await; // Check if all tiles exist
    println!("Tiles exist: {}", tiles_exist);
    if tiles_exist {
        // If they do, get them from the database
        let flat_tiles = db::get_many(x_range, y_range).await;
        let mut tile_row: Vec<Tile> = Vec::new();
        let mut i = 0;
        println!("{} tiles", flat_tiles.len());
        for tile in flat_tiles {
            i += 1;
            tile_row.push(tile);
            if i % image::VIEW_DISTANCE == 0 {
                tiles.push(tile_row);
                tile_row = Vec::new();
                i = 0;
            }
        }
    } else {
        // If they don't, get the tiles we can from the database and generate the rest
        let database = db::get_db().await;
        for x in offset_x_min..offset_x_max {
            let mut tile_row: Vec<Tile> = Vec::new();
            for y in offset_y_min..offset_y_max {
                let current_tile: Tile;
                if !db::internal_check_tile(&database, x, y).await {
                    current_tile = blank_tile().await;
                } else {
                    current_tile = db::internal_get_tile(&database, x, y).await;
                }
                tile_row.push(current_tile);
            }
            tiles.push(tile_row);
        }
    }
    let start = std::time::Instant::now();
    let image = image::draw_map(&tiles).await;
    println!("Map drawn in {}ms", start.elapsed().as_millis()); // Just for debugging, will be removed later
    image.save("map.jpg").unwrap();
    let attachment = AttachmentType::File {
        file: &File::open("map.jpg").await?,
        filename: "map.jpg".to_string(),
    };

    ctx.send(|b| b.attachment(attachment)).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    println!("Starting bot...");
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![map()], // for some reason intellij is complaining about this line, but it works fine
            ..Default::default()
        })
        .token(config::get_config().discord_token)
        .intents(serenity::GatewayIntents::all())
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                ctx.set_activity(serenity::Activity::playing("with the map"))
                    .await;
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                println!("Bot started.");
                Ok(Data {})
            })
        });
    framework.run().await.unwrap();
    println!("Bot stopped.")
}