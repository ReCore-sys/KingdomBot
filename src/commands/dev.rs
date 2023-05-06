use crate::db::tiles;
use crate::db::tiles::blank_tile;
use crate::misc::reply_admin;
use crate::{db, Context, Error};

#[poise::command(
    slash_command,
    prefix_command,
    ephemeral,
    description_localized("en-US", "A set of dev commands"),
    subcommands(
        "create_tile",
        "update_fields",
        "clean_db",
        "panic",
        "error",
        "build_db"
    )
)]
pub(crate) async fn dev(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("This command is not yet implemented").await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub(crate) async fn create_tile(
    ctx: Context<'_>,
    #[description = "X coordinate of the tile"] x: i32,
    #[description = "Y coordinate of the tile"] y: i32,
) -> Result<(), Error> {
    if !reply_admin(ctx).await? {
        return Ok(());
    }
    let tile = blank_tile(x, y).await;
    tiles::set_tile(tile).await.expect("Failed to set tile");
    ctx.say("Tile created").await?;

    Ok(())
}

#[poise::command(
    slash_command,
    prefix_command,
    ephemeral,
    description_localized("en-US", "Update struct fields in the db")
)]
pub(crate) async fn update_fields(ctx: Context<'_>) -> Result<(), Error> {
    if !reply_admin(ctx).await? {
        return Ok(());
    }
    let tiles = tiles::get_all().await?;
    tiles::set_many(tiles).await.expect("Failed to set tiles");
    let users = db::users::get_all().await?;
    db::users::set_many(users)
        .await
        .expect("Failed to set users");
    let factions = db::factions::get_all().await?;
    db::factions::set_many(factions)
        .await
        .expect("Failed to set factions");
    ctx.say("Updated fields").await?;

    Ok(())
}

#[poise::command(
    slash_command,
    prefix_command,
    ephemeral,
    description_localized("en-US", "Cleans the database")
)]
pub(crate) async fn clean_db(ctx: Context<'_>) -> Result<(), Error> {
    if !reply_admin(ctx).await? {
        return Ok(());
    }
    db::cleaners::clean_factions().await?;
    db::cleaners::clean_tiles().await?;
    db::cleaners::clean_users().await?;
    ctx.say("Database is nice and squeaky clean!").await?;
    Ok(())
}

#[poise::command(
    slash_command,
    prefix_command,
    ephemeral,
    description_localized("en-US", "Causes a division by zero panic")
)]
pub(crate) async fn panic(ctx: Context<'_>) -> Result<(), Error> {
    if !reply_admin(ctx).await? {
        return Ok(());
    }
    let _ = 5 / 0;
    Ok(())
}

#[poise::command(
    slash_command,
    prefix_command,
    ephemeral,
    description_localized("en-US", "Returns an error")
)]
pub(crate) async fn error(ctx: Context<'_>) -> Result<(), Error> {
    if !reply_admin(ctx).await? {
        return Ok(());
    }
    let error = Error::from("This is an error");
    Err(error)
}

#[poise::command(
    slash_command,
    prefix_command,
    ephemeral,
    description_localized("en-US", "Rebuild a faction's production stats")
)]
pub(crate) async fn build_db(ctx: Context<'_>) -> Result<(), Error> {
    db::build_production().await?;
    ctx.say("Rebuilt production stats").await?;
    Ok(())
}