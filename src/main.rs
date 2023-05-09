#[macro_use]
extern crate log;

use poise::serenity_prelude as serenity;
use rust_embed::RustEmbed;
use tokio::join;

use crate::background::background_loop;
use commands::map::map;

use crate::commands::build::build;
use crate::commands::dev::dev;
use crate::commands::faction::faction;
use crate::commands::help::{explain, guide};
use crate::commands::r#move::move_troops;
use crate::commands::tile::tile;
use crate::commands::user::register;

#[path = "utils/background.rs"]
mod background;
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
    let bot_task = tokio::task::spawn(framework.run());
    let bg_loop = tokio::task::spawn(background_loop());
    let loop_res = join!(bot_task, bg_loop);
    loop_res
        .0
        .expect("Main bot task broke in at the task level")
        .expect("The bot task broke at the bot level");
    loop_res.1.expect("Background loop broke");
    warn!("Bot stopped.")
}