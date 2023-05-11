use crate::conversions::convert_user;
use crate::db;
use crate::misc::log_command_used;
use crate::{Context, Error};

#[poise::command(
slash_command,

// ephemeral,
description_localized("en-US", "Register yourself to play the game!")
)]
pub(crate) async fn register(ctx: Context<'_>) -> Result<(), Error> {
    log_command_used(ctx).await;
    if db::users::user_exists(ctx.author().id.to_string()).await? {
        ctx.say("You are already registered!").await?;
        return Ok(());
    }
    let converted_user = convert_user(ctx.author()).await;
    db::users::save_user(converted_user)
        .await
        .expect("Failed to save user");
    ctx.say(
        "*And so your story begins...*\n\nIf this is \
    your first time playing, you can use **/guide** to get a quick rundown on how to play.",
    )
    .await?;
    return Ok(());
}