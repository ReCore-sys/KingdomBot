use crate::conversions::convert_user;
use crate::db;
use crate::{Context, Error};

#[poise::command(slash_command, prefix_command, ephemeral)]
pub(crate) async fn register(ctx: Context<'_>) -> Result<(), Error> {
    if db::users::user_exists(ctx.author().id.to_string()).await {
        ctx.say("You are already registered!").await?;
        return Ok(());
    }
    let converted_user = convert_user(ctx.author()).await;
    db::users::save_user(converted_user).await;
    ctx.say("You have been registered!").await?;
    return Ok(());
}