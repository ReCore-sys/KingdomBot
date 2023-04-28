#[macro_use]
extern crate log;

use poise::serenity_prelude as serenity;
use rust_embed::RustEmbed;

use commands::map::map;

use crate::commands::build::build;
use crate::commands::dev::dev;
use crate::commands::faction::faction;
use crate::commands::help::{explain, guide};
use crate::commands::r#move::move_troops;
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
#[path = "utils/misc_utils.rs"]
mod misc;
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
    pretty_env_logger::init();
    info!("Starting bot...");
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                map(),
                dev(),
                guide(),
                explain(),
                tile(),
                move_troops(),
                build(),
                faction(),
                register(),
            ], // for some reason intellij is complaining about this line, but it works fine
            on_error: |error| {
                Box::pin(async move {
                    error
                        .ctx()
                        .unwrap()
                        .say(format!("Error: {}", error))
                        .await
                        .expect("Shit has really hit the fan");
                    error!("Error: {}", error)
                })
            },
            ..Default::default()
        })
        .token(config::get_config().discord_token)
        .intents(serenity::GatewayIntents::all())
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                ctx.set_activity(serenity::Activity::playing("with the map"))
                    .await;
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                info!("Bot started.");
                Ok(Data {})
            })
        });
    framework.run().await.unwrap();
    warn!("Bot stopped.")
}