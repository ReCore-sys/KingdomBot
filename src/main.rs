use poise::serenity_prelude as serenity;

use crate::commands::map::create_tile;
use commands::map::map;

mod commands;
#[path = "utils/config.rs"]
mod config;
#[path = "utils/db.rs"]
mod db;
#[path = "utils/image.rs"]
mod image;
#[path = "utils/types/map.rs"]
mod map;
mod tests;

struct Data {}

// User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() {
    println!("Starting bot...");
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![map(), create_tile()], // for some reason intellij is complaining about this line, but it works fine
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