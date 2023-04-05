use crate::{db, Context, Error};

#[poise::command(slash_command, prefix_command, subcommands("info"))]
pub(crate) async fn tile(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}

#[poise::command(
    slash_command,
    prefix_command,
    ephemeral,
    description_localized("en-US", "Gets info about a tile")
)]
pub(crate) async fn info(ctx: Context<'_>, x: i32, y: i32) -> Result<(), Error> {
    let cant_see_message = "You can't see that tile!";
    let faction = db::users::get_user(ctx.author().id.to_string())
        .await
        .faction;
    let can_see = db::tiles::can_faction_see(x, y, faction).await;
    if !can_see {
        ctx.say(cant_see_message).await?;
        return Ok(());
    }
    let tile = db::tiles::get_tile(x, y).await;
    let mut owner = "None".to_string();
    if tile.faction != "" {
        owner = db::factions::get_faction(tile.faction.clone())
            .await
            .name
            .clone();
    }
    let mut buildings = "".to_string();
    for (k, v) in tile.buildings {
        buildings.push_str(format!("{}: {}\n", k.data().name, v).as_str());
    }
    if buildings == "" {
        buildings = "None".to_string();
    }
    let mut units = "".to_string();
    for (k, v) in tile.units {
        units.push_str(format!("{}: {}\n", k.data().name, v).as_str());
    }
    if units == "" {
        units = "None".to_string();
    }
    ctx.send(|b| {
        b.embed(|e| {
            e.title(format!("{}, {}", x, y))
                .field("Owner", owner, false)
                .field("Buildings", buildings, false)
                .field("Units", units, false)
        })
    })
    .await?;

    Ok(())
}