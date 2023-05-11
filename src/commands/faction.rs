use std::time::SystemTime;

use poise::Modal;
use rand::Rng;
use regex::Regex;
use tokio::time;

use crate::conversions::modal_to_faction;
use crate::db::tiles::blank_tile;
use crate::image::VIEW_DISTANCE;
use crate::misc::log_command_used;
use crate::types::buildings::Building;
use crate::types::permissions::Permissions;
use crate::types::units::Unit;
use crate::{db, Context, Data, Error};

const CAPITAL_PLACE_RANGE: i32 = VIEW_DISTANCE * 3;
const INFO_INLINE: bool = true;
const MAX_CAPITAL_DIST: i32 = 500;

type ApplicationContext<'a> = poise::ApplicationContext<'a, Data, Error>;

#[poise::command(slash_command, subcommands("create", "info"))]
pub(crate) async fn faction(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}

#[derive(Debug, poise::Modal, Clone)]
pub(crate) struct FactionModal {
    #[name = "The display name of the faction"]
    #[placeholder = "A really cool faction name"]
    #[max_length = 50]
    pub(crate) faction_name: String,
    #[name = "The 4 character tag of the faction"]
    #[max_length = 4]
    #[min_length = 4]
    #[placeholder = "ABCD"]
    pub(crate) faction_tag: String,
    #[max_length = 250]
    #[name = "The description of the faction"]
    #[placeholder = "How would you describe your faction? This can be changed later"]
    #[paragraph]
    pub(crate) faction_description: String,
    #[name = "The location of the faction's capital"]
    #[placeholder = "0, 0 \n(Leave blank for a random location)"]
    pub(crate) faction_location: Option<String>,
}

#[poise::command(
slash_command,
// ephemeral,
description_localized("en-US", "If you aren't already in one, create a faction!")
)]
pub(crate) async fn create(ctx: ApplicationContext<'_>) -> Result<(), Error> {
    log_command_used(ctx).await;
    if !db::users::user_exists(ctx.author().id.to_string()).await? {
        ctx.say("You need to register first!\nUse `/register` to join!")
            .await?;
        return Ok(());
    }
    if db::users::get_user(ctx.author().id.to_string())
        .await?
        .faction
        != ""
    {
        ctx.say("You are already in a faction!").await?;
        return Ok(());
    }
    let data = FactionModal::execute(ctx).await?.unwrap();
    let pattern = Regex::new(r"(-?\d+)[,|\s]*(-?\d+)").unwrap();
    let location_chosen = data.faction_location.is_some();
    if location_chosen && !pattern.is_match(&data.faction_location.as_ref().unwrap()) {
        ctx.say("Invalid location! Needs to be in the format of `x, y` where x and y are positive or negative integers")
            .await?;
        return Ok(());
    }
    #[allow(unused_assignments)]
    let mut faction_location = (0, 0);
    if location_chosen {
        let captures = pattern
            .captures(&data.faction_location.as_ref().unwrap())
            .unwrap();
        let x = captures.get(1).unwrap().as_str().parse::<i32>().unwrap();
        if x > MAX_CAPITAL_DIST || x < -MAX_CAPITAL_DIST {
            ctx.say(format!(
                "The x coordinate must be between {} and {}!",
                -MAX_CAPITAL_DIST, MAX_CAPITAL_DIST
            ))
            .await?;
            return Ok(());
        }
        let y = captures.get(2).unwrap().as_str().parse::<i32>().unwrap();
        if y > MAX_CAPITAL_DIST || y < -MAX_CAPITAL_DIST {
            ctx.say(format!(
                "The y coordinate must be between {} and {}!",
                -MAX_CAPITAL_DIST, MAX_CAPITAL_DIST
            ))
            .await?;
            return Ok(());
        }
        let y_range = (y + CAPITAL_PLACE_RANGE, y - CAPITAL_PLACE_RANGE);
        let x_range = (x + CAPITAL_PLACE_RANGE, x - CAPITAL_PLACE_RANGE);
        let valid = !db::tiles::any_exist(x_range, y_range).await?;
        if !valid {
            ctx.say("That location is too close to an existing faction!")
                .await?;
            return Ok(());
        } else {
            faction_location = (x, y);
        }
    } else {
        let mut distance = CAPITAL_PLACE_RANGE;
        // get a random position `distance` tiles from 0,0 and increment it each time there are any taken tiles in range
        let mut random_generator = rand::rngs::OsRng;
        loop {
            let horizontal = random_generator.gen_range(0..2) == 0;
            let top_or_left = random_generator.gen_range(0..2) == 0;
            if horizontal {
                let x = if top_or_left { distance } else { -distance };
                let y = random_generator.gen_range(-distance..distance);
                let y_range = (y + CAPITAL_PLACE_RANGE, y - CAPITAL_PLACE_RANGE);
                let x_range = (x + CAPITAL_PLACE_RANGE, x - CAPITAL_PLACE_RANGE);
                let valid = !db::tiles::any_exist(x_range, y_range).await?;
                if valid {
                    faction_location = (x, y);
                    break;
                } else {
                    distance += 1;
                }
            } else {
                let x = random_generator.gen_range(-distance..distance);
                let y = if top_or_left { distance } else { -distance };
                let y_range = (y + CAPITAL_PLACE_RANGE, y - CAPITAL_PLACE_RANGE);
                let x_range = (x + CAPITAL_PLACE_RANGE, x - CAPITAL_PLACE_RANGE);
                let valid = !db::tiles::any_exist(x_range, y_range).await?;
                if valid {
                    faction_location = (x, y);
                    break;
                } else {
                    distance += 1;
                }
            }
        }
    }
    let tag = &data.faction_tag.to_uppercase();
    if db::factions::faction_exists(tag.clone()).await? {
        ctx.say("That faction tag is already taken!").await?;
        return Ok(());
    }
    let mut converted_data = modal_to_faction(&data).await;
    converted_data.leader = ctx.author().id.to_string();
    converted_data.production.money = 100.0;
    converted_data.production.wood = 100.0;
    converted_data.production.metal = 100.0;
    converted_data.production.food = 500.0;
    converted_data.production.population = 100.0;
    converted_data.production.happiness = 80.0;
    converted_data.capital_x = faction_location.0;
    converted_data.capital_y = faction_location.1;
    converted_data.production.last_updated = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    converted_data.members.push(ctx.author().id.to_string());
    db::factions::save_faction(converted_data)
        .await
        .expect("Failed to save faction");
    let mut faction_tile = blank_tile(faction_location.0, faction_location.1).await;
    faction_tile.faction = tag.clone();
    faction_tile.occupied = true;
    faction_tile.units.insert(Unit::Citizen, 100);
    let insert_result = faction_tile.buildings.insert(Building::Capital, 1);
    if insert_result.is_some() {
        panic!(
            "Failed to insert building into tile! Capital already exists? Tried to set at {}, {}",
            faction_location.0, faction_location.1
        );
    }
    db::tiles::set_tile(faction_tile)
        .await
        .expect("Failed to save tile");
    let message = format!("*The seeds of a mighty empire have been sown...*\n\nYour capital has been built at \
    {}, {} and a small group of settlers have moved in. \
     You need to grow and defend your empire, but to do that you will need buildings, troops and resources to keep your \
     people happy and to fend off the other factions.\n\nBest of luck and may you prosper!", faction_location.0, faction_location.1);

    ctx.say(message).await?;
    let mut user = db::users::get_user(ctx.author().id.to_string()).await?;
    user.faction = tag.clone();
    user.permissions.push(Permissions::Leader);
    db::users::save_user(user)
        .await
        .expect("Failed to save user");
    return Ok(());
}

#[poise::command(
slash_command,
// ephemeral,
description_localized("en-US", "Get information about your faction")
)]
pub(crate) async fn info(ctx: Context<'_>) -> Result<(), Error> {
    log_command_used(ctx).await;
    let user = db::users::get_user(ctx.author().id.to_string()).await?;
    if user.faction == "" {
        ctx.say("You are not in a faction!").await?;
        return Ok(());
    }
    let faction = db::factions::get_faction(user.faction.clone()).await?;

    let leader = db::users::get_user(faction.leader.clone()).await?;

    ctx.send(|e| {
        e.embed(|embed| {
            embed
                .title(faction.name)
                .description(faction.description)
                .field("Tag", faction.tag, INFO_INLINE)
                .field(
                    "Capital location",
                    format!("{}, {}", faction.capital_x, faction.capital_y),
                    INFO_INLINE,
                )
                .field("Leader", leader.username, INFO_INLINE)
                .field(
                    "Population",
                    format!("{}", faction.production.population.floor() as i32),
                    INFO_INLINE,
                )
                .field(
                    "Money",
                    format!("${:.2}", faction.production.money),
                    INFO_INLINE,
                )
                .field(
                    "Food",
                    format!("{:.2}kg", faction.production.food),
                    INFO_INLINE,
                )
                .field(
                    "Wood",
                    format!("{:}", faction.production.wood.floor() as i32),
                    INFO_INLINE,
                )
                .field(
                    "Metal",
                    format!("{}", faction.production.metal.floor() as i32),
                    INFO_INLINE,
                )
        })
    })
    .await?;

    return Ok(());
}