use poise::Modal;

use crate::conversions::modal_to_faction;
use crate::{db, Context, Data, Error};

type ApplicationContext<'a> = poise::ApplicationContext<'a, Data, Error>;

#[poise::command(slash_command, prefix_command, subcommands("create"))]
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
    converted_data.money = 100.0;
    db::factions::save_faction(converted_data)
        .await
        .expect("Failed to save faction");
    ctx.say("Faction created!").await?;
    let mut user = db::users::get_user(ctx.author().id.to_string()).await;
    user.faction = tag;
    db::users::save_user(user)
        .await
        .expect("Failed to save user");
    return Ok(());
}