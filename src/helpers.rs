use poise::CreateReply;

use crate::{Context, Error};

pub async fn is_add_authorized(ctx: Context<'_>) -> Result<bool, Error> {
    if !ctx.data().add_users.contains(&ctx.author().id.get()) {
        ctx.send(CreateReply::default().content("You are not authorized to use this command.").ephemeral(true)).await?;
        return Ok(false);
    }
    Ok(true)
}