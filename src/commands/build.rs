use crate::conversions::string_to_building;
use crate::types::buildings::Building;
use crate::types::permissions::Permissions;
use crate::{db, Context, Error};

#[poise::command(
    slash_command,
    prefix_command,
    ephemeral,
    description_localized("en-US", "Construct a building")
)]
pub(crate) async fn build(
    ctx: Context<'_>,
    #[description_localized("en-US", "The building to build")] building: String,
    #[description_localized("en-US", "The amount of buildings to build")] amount: i32,
    #[description_localized("en-US", "The x coordinate of the tile to build on")] x: i32,
    #[description_localized("en-US", "The y coordinate of the tile to build on")] y: i32,
) -> Result<(), Error> {
    let user = db::users::get_user(ctx.author().id.to_string()).await?;
    if !user.permitted(Permissions::Build) {
        ctx.say("You don't have permission to build!").await?;
        return Ok(());
    }

    if amount == 0 {
        ctx.say("You can't build nothing!").await?;
        return Ok(());
    }
    if amount < 0 {
        ctx.say("You can't build a negative amount of buildings!")
            .await?;
        return Ok(());
    }
    // Check that the tile isn't already owned by someone else
    let tile_exists = db::tiles::check_tile(x, y).await?;
    if tile_exists {
        let tile = db::tiles::get_tile(x, y).await?;
        let faction = db::users::get_user(ctx.author().id.to_string())
            .await?
            .faction;
        if !tile.faction.is_empty() && tile.faction != faction {
            ctx.say("You can't build on tiles that aren't yours!")
                .await?;
            return Ok(());
        }
    }
    let possible_building = string_to_building(&building.to_lowercase()).await;
    match possible_building.is_err() {
        true => {
            ctx.say("That's not a valid building!").await?;
            return Ok(());
        }
        false => (),
    }
    if possible_building.clone().unwrap() == Building::Capital {
        ctx.say("You can't build a capital!").await?;
        return Ok(());
    }
    let tile = db::tiles::get_tile(x, y).await?;
    let existing_buildings = tile.buildings;
    let mut used_space = 0;
    for (building, amount) in existing_buildings {
        used_space += building.data().space * amount as i32;
    }
    let building = possible_building.unwrap();
    let building_space = building.data().space;
    if amount * building_space > 100 - used_space {
        if amount == 1 {
            ctx.say("You don't have enough space to build that building!")
                .await?;
        } else {
            ctx.say("You don't have enough space to build that many buildings!")
                .await?;
        }
        return Ok(());
    }

    // TODO finish this

    Ok(())
}