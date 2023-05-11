use crate::conversions::string_to_unit;
use crate::misc::log_command_used;
use crate::types::permissions::Permissions;
use crate::types::units::Unit;
use crate::{db, Context, Error};

#[poise::command(
slash_command,

// ephemeral,
description_localized("en-US", "Move troops and people from one position to another")
)]
pub(crate) async fn move_troops(
    ctx: Context<'_>,
    #[description = "The x coordinate of the tile you want to move from"] from_x: i32,
    #[description = "The y coordinate of the tile you want to move from"] from_y: i32,
    #[description = "The x coordinate of the tile you want to move to"] to_x: i32,
    #[description = "The y coordinate of the tile you want to move to"] to_y: i32,
    #[description = "The name of the unit you want to move. Check \"/explain units\" for a list of units"]
    unit: String,
    #[description = "The amount of units you want to move"] amount: u32,
) -> Result<(), Error> {
    log_command_used(ctx).await;
    let user = db::users::get_user(ctx.author().id.to_string()).await?;
    if !user.permitted(Permissions::MoveTroops) {
        ctx.say("You don't have permission to move troops!").await?;
        return Ok(());
    }
    if amount == 0 {
        ctx.say("You can't move 0 units!").await?;
        return Ok(());
    }
    let bad_tile_message = "You don't have any of that unit on that tile!";
    let tile_exists = db::tiles::check_tile(from_x, from_y).await?;
    if !tile_exists {
        ctx.say(bad_tile_message).await?;
        return Ok(());
    }
    let faction = db::users::get_user(ctx.author().id.to_string())
        .await?
        .faction;
    let can_see = db::tiles::can_faction_see(to_x, to_y, faction.clone()).await?;
    if !can_see {
        ctx.say("You can't move units into tiles beyond your reach!")
            .await?;
        return Ok(());
    }
    let tile = db::tiles::get_tile(from_x, from_y).await?;
    if tile.faction != faction {
        ctx.say(bad_tile_message).await?;
        return Ok(());
    }
    let units = tile.units;
    let possible_unit = string_to_unit(&unit.to_lowercase()).await;
    if possible_unit.is_err() {
        ctx.say("That's not a valid unit!").await?;
        return Ok(());
    }
    if !units.contains_key(&possible_unit.clone().unwrap()) {
        ctx.say(bad_tile_message).await?;
        return Ok(());
    }
    let unit_amount = units.get(&possible_unit.unwrap());
    if unit_amount.is_none() {
        // We already checked if the unit exists, so this should never happen
        // But just in case...
        ctx.say(bad_tile_message).await?;
        return Ok(());
    }
    if amount > *unit_amount.unwrap() {
        ctx.say("You don't have that many units on that tile!")
            .await?;
        return Ok(());
    }

    // Using pythagorean theorem to calculate distance
    let distance_precise = (((from_x - to_x).pow(2) + (from_y - to_y).pow(2)) as f64).sqrt();
    let distance = distance_precise.ceil() as u32;
    let mut food_cost = 0;
    for (unit, amount) in units.clone() {
        if unit == Unit::Scout || unit == Unit::Soldier {
            food_cost += amount;
        } else if unit == Unit::Cavalry {
            food_cost += amount * 2;
        } else if unit == Unit::Ranger {
            food_cost += amount * 3;
        } else if unit == Unit::Knight {
            food_cost += amount * 4;
        }
    }
    food_cost *= distance;
    if food_cost == 0 {
        food_cost = 1;
    }
    let food = db::factions::get_faction(faction.clone())
        .await?
        .production
        .food;
    if food < food_cost as f32 {
        ctx.send(|e| {
            e.embed(|e| {
                e.title("You don't have enough food to move that many units!");
                e.description(format!(
                    "You need {} food, but you only have {} food.",
                    food_cost, food
                ));
                e
            });
            e
        })
        .await?;
        return Ok(());
    }
    // Passed all the checks, somehow

    // Remove units from old tile
    let mut from_tile = db::tiles::get_tile(from_x, from_y).await?;
    let mut to_tile = db::tiles::get_tile(to_x, to_y).await?;
    for (unit, _) in from_tile.units.clone() {
        from_tile
            .units
            .insert(unit.clone(), from_tile.units.get(&unit).unwrap() - amount);
        if from_tile.units.get(&unit).unwrap() == &0 {
            from_tile.units.remove(&unit);
        }
        let unit_exists = to_tile.units.contains_key(&unit);

        let new_amount = if unit_exists {
            to_tile.units.get(&unit).unwrap() + amount
        } else {
            amount
        };
        to_tile.units.insert(unit.clone(), new_amount);
    }
    let mut save_result = db::tiles::set_tile(from_tile).await;
    if save_result.is_err() {
        ctx.say("Something went wrong while saving the tile!")
            .await?;
        return Ok(());
    }
    to_tile.faction = faction.clone();
    to_tile.occupied = true;
    save_result = db::tiles::set_tile(to_tile).await;
    if save_result.is_err() {
        ctx.say("Something went wrong while saving the tile!")
            .await?;
        return Ok(());
    }
    let current_food = db::factions::get_faction(faction.clone())
        .await?
        .production
        .food;
    let new_food = current_food - food_cost as f32;
    let mut faction = db::factions::get_faction(faction.clone()).await?;
    faction.production.food = new_food;
    save_result = db::factions::save_faction(faction).await;
    if save_result.is_err() {
        ctx.say("Something went wrong while saving the faction!")
            .await?;
        return Ok(());
    }
    ctx.send(|e| {
        e.embed(|e| {
            e.title("Moved units!");
            e.description(format!(
                "Moved {} {} from {} {} to {} {}. Food cost: {}",
                amount, unit, from_x, from_y, to_x, to_y, food_cost
            ));
            e
        });
        e
    })
    .await
    .unwrap();

    Ok(())
}