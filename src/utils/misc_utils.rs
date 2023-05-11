use crate::{Context, Error};

pub(crate) async fn reply_admin(ctx: Context<'_>) -> Result<bool, Error> {
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
        return Ok(false);
    }
    Ok(true)
}

pub async fn log_command_used(ctx: Context<'_>) {
    let mut cmd_name = "".to_string();
    if ctx.parent_commands().len() > 0 {
        cmd_name = ctx.parent_commands()[0].name.clone();
        cmd_name += " ";
    }
    cmd_name = cmd_name + &ctx.command().name;
    let mut param = "".to_string()
        + &ctx
            .command()
            .parameters
            .iter()
            .map(|x| x.name.clone())
            .collect::<Vec<String>>()
            .join(" ")
        + " ";
    if ctx.command().parameters.len() == 0 {
        param = "".to_string();
    }
    info!(
        "Command used: {} {}: {}",
        cmd_name,
        param,
        ctx.author().name
    );
}