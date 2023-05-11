use std::str;

use regex::Regex;

use crate::misc::log_command_used;
use crate::{Context, Error};

#[poise::command(
    slash_command,
    description_localized("en-US", "Walks you through how the game works")
)]
pub(crate) async fn guide(ctx: Context<'_>) -> Result<(), Error> {
    log_command_used(ctx).await;
    ctx.say(crate::GUIDE_MESSAGE).await?;
    return Ok(());
}

#[poise::command(
slash_command,

// ephemeral,
description_localized("en-US", "Explains a topic")
)]
pub(crate) async fn explain(
    ctx: Context<'_>,
    #[name_localized("en-US", "topic")]
    #[description = "The topic to explain. Leave blank to list the topics"]
    command: Option<String>,
) -> Result<(), Error> {
    log_command_used(ctx).await;
    let pattern = Regex::new(r"(.+)\.txt").unwrap();
    let mut topics: Vec<String> = Vec::new();
    for topic in crate::HelpTopics::iter() {
        topics.push(
            pattern
                .captures(&topic)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .to_string(),
        );
    }
    if command.is_none() {
        ctx.say(format!("Available topics: {}", topics.join(", ")))
            .await?;
        return Ok(());
    }
    let topic_name = command.unwrap().to_lowercase();
    if !topics.contains(&topic_name) {
        ctx.say(format!(
            "Invalid topic. Available topics: {}",
            topics.join(", ")
        ))
        .await?;
        return Ok(());
    }
    let topic_path = format!("{}.txt", topic_name);
    let topic = crate::HelpTopics::get(&topic_path).unwrap();
    ctx.say(format!(
        "**{}**\n{}",
        topic_name,
        str::from_utf8(topic.data.as_ref()).unwrap()
    ))
    .await?;

    Ok(())
}