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
    let user = db::users::get_user(ctx.author().id.to_string())
        .await
        .unwrap();
    if !user.permitted(Permissions::Build) {
        ctx.say("You don't have permission to build!")
            .await
            .unwrap();
        return Ok(());
    }

    if amount == 0 {
        ctx.say("You can't build nothing!").await.unwrap();
        return Ok(());
    }
    if amount < 0 {
        ctx.say("You can't build a negative amount of buildings!")
            .await
            .unwrap();
        return Ok(());
    }
    // Check that the tile isn't already owned by someone else
    let tile_exists = db::tiles::check_tile(x, y).await.unwrap();
    if tile_exists {
        let tile = db::tiles::get_tile(x, y).await.unwrap();
        let faction = db::users::get_user(ctx.author().id.to_string())
            .await
            .unwrap()
            .faction;
        if !tile.faction.is_empty() && tile.faction != faction {
            ctx.say("You can't build on tiles that aren't yours!")
                .await
                .unwrap();
            return Ok(());
        }
    }
    let possible_building = string_to_building(&building.to_lowercase()).await;
    match possible_building.is_err() {
        true => {
            ctx.say("That's not a valid building!").await.unwrap();
            return Ok(());
        }
        false => (),
    }
    if possible_building.clone().unwrap() == Building::Capital {
        ctx.say("You can't build a capital!").await.unwrap();
        return Ok(());
    }
    let tile = db::tiles::get_tile(x, y).await.unwrap();
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
                .await
                .unwrap();
        } else {
            ctx.say("You don't have enough space to build that many buildings!")
                .await
                .unwrap();
        }
        return Ok(());
    }
    let faction_tag = db::users::get_user(ctx.author().id.to_string())
        .await
        .unwrap()
        .faction;
    let mut faction = db::factions::get_faction(faction_tag).await.unwrap();
    let details = building.data();
    if details.cost > faction.production.money.floor() as i32 {
        ctx.say("You don't have enough money to build that!")
            .await
            .unwrap();
        return Ok(());
    }
    if details.wood > faction.production.wood.floor() as i32 {
        ctx.say("You don't have enough wood to build that!")
            .await
            .unwrap();
        return Ok(());
    }
    if details.metal > faction.production.metal.floor() as i32 {
        ctx.say("You don't have enough metal to build that!")
            .await
            .unwrap();
        return Ok(());
    }

    faction.production.money -= details.cost as f32;
    faction.production.wood -= details.wood as f32;
    faction.production.metal -= details.metal as f32;
    db::factions::save_faction(faction).await.unwrap();
    let mut tile = db::tiles::get_tile(x, y).await.unwrap();
    let mut buildings = tile.buildings;
    match buildings.get(&building) {
        Some(existing_buildings) => {
            buildings.insert(building, existing_buildings + amount as u32);
        }
        None => {
            buildings.insert(building, amount as u32);
        }
    }
    tile.buildings = buildings;
    db::tiles::set_tile(tile).await.unwrap();
    ctx.say(format!("You built {} {}s!", amount, details.name))
        .await
        .unwrap();

    Ok(())
}