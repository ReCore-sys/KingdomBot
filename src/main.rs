use poise::serenity_prelude as serenity;
use rust_embed::RustEmbed;

use commands::map::map;

use crate::commands::faction::faction;
use crate::commands::help::{explain, guide};
use crate::commands::map::create_tile;
use crate::commands::tile::tile;
use crate::commands::user::register;

mod commands;
#[path = "utils/config.rs"]
mod config;
#[path = "utils/conversions.rs"]
mod conversions;
#[path = "utils/db.rs"]
mod db;
#[path = "utils/image.rs"]
mod image;
mod tests;
#[path = "utils/types.rs"]
mod types;

#[derive(RustEmbed)]
#[folder = "src/help/topics/"]
struct HelpTopics;

struct Data {}

// User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

const GUIDE_MESSAGE: &str = include_str!("help/guide.txt");

#[tokio::main]
async fn main() {
    println!("Starting bot...");
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                map(),
                create_tile(),
                faction(),
                register(),
                guide(),
                explain(),
                tile(),
            ], // for some reason intellij is complaining about this line, but it works fine
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