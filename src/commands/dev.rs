use crate::db::tiles;
use crate::db::tiles::blank_tile;
use crate::{db, Context, Error};

#[poise::command(
    slash_command,
    prefix_command,
    ephemeral,
    description_localized("en-US", "A set of dev commands"),
    subcommands("create_tile", "update_fields")
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

#[poise::command(
    slash_command,
    prefix_command,
    ephemeral,
    description_localized("en-US", "Update struct fields in the db")
)]
pub(crate) async fn update_fields(ctx: Context<'_>) -> Result<(), Error> {
    if !ctx
        .author_member()
        .await
        .unwrap()
        .permissions
        .unwrap()
        .administrator()
    {
        ctx.say("You do not have permission to use this command")
            .await?;
        return Ok(());
    }
    let tiles = db::tiles::get_all().await?;
    db::tiles::set_many(tiles)
        .await
        .expect("Failed to set tiles");
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