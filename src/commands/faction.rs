


use poise::{Modal};

use crate::conversions::modal_to_faction;
use crate::{db, Context, Data, Error};

type ApplicationContext<'a> = poise::ApplicationContext<'a, Data, Error>;

#[poise::command(slash_command, prefix_command, subcommands("create", "info"))]
pub(crate) async fn faction(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}

#[derive(Debug, poise::Modal)]
pub(crate) struct FactionModal {
    #[name = "The display name of the faction"]
    #[placeholder = "A really cool faction name"]
    pub(crate) faction_name: String,
    #[name = "The 4 character tag of the faction"]
    #[max_length = 4]
    #[min_length = 4]
    #[placeholder = "ABCD"]
    pub(crate) faction_tag: String,
    #[name = "The description of the faction"]
    #[placeholder = "How would you describe your faction? This can be changed later"]
    #[paragraph]
    pub(crate) faction_description: String,
}

#[poise::command(slash_command, ephemeral)]
pub(crate) async fn create(ctx: ApplicationContext<'_>) -> Result<(), Error> {
    if !db::users::user_exists(ctx.author().id.to_string()).await {
        ctx.say("You need to register first!\nUse `/register` to join!")
            .await?;
        return Ok(());
    }
    if db::users::get_user(ctx.author().id.to_string())
        .await
        .faction
        != ""
    {
        ctx.say("You are already in a faction!").await?;
        return Ok(());
    }
    let data = FactionModal::execute(ctx).await?.unwrap();
    let tag = data.faction_tag.clone();
    if db::factions::faction_exists(tag.clone()).await {
        ctx.say("That faction tag is already taken!").await?;
        return Ok(());
    }
    let mut converted_data = modal_to_faction(data).await;
    converted_data.leader = ctx.author().id.to_string();
    converted_data.production.money = 100.0;
    converted_data.production.food = 100.0;
    converted_data.production.population = 100;
    converted_data.production.happiness = 80.0;
    db::factions::save_faction(converted_data)
        .await
        .expect("Failed to save faction");
    let message = "*The seeds of a mighty empire have been sown...*\n\nFeel free to use the **/info** command to get \
    more information on your faction, or **/help** to see important info and commands.";
    ctx.say(message).await?;
    let mut user = db::users::get_user(ctx.author().id.to_string()).await;
    user.faction = tag;
    db::users::save_user(user)
        .await
        .expect("Failed to save user");
    return Ok(());
}

#[poise::command(slash_command, prefix_command, ephemeral)]
pub(crate) async fn info(ctx: Context<'_>) -> Result<(), Error> {
    let user = db::users::get_user(ctx.author().id.to_string()).await;
    if user.faction == "" {
        ctx.say("You are not in a faction!").await?;
        return Ok(());
    }
    let faction = db::factions::get_faction(user.faction.clone()).await;

    let leader = db::users::get_user(faction.leader.clone()).await;

    ctx.send(|e| {
        e.embed(|embed| {
            embed
                .title(faction.name)
                .description(faction.description)
                .field("Tag", faction.tag, true)
                .field("Leader", leader.username, true)
                .field(
                    "Population",
                    faction.production.population.to_string(),
                    true,
                )
                .field("Money", format!("${:.2}", faction.production.money), true)
                .field("Food", format!("{}kg", faction.production.food), true)
        })
    })
    .await?;

    return Ok(());
}