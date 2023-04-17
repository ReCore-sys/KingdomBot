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