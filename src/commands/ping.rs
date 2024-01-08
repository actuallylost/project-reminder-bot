use crate::{Context, Error};

// Ping the bot!
#[poise::command(slash_command, prefix_command)]
pub async fn ping(
    ctx: Context<'_>,
) -> Result<(), Error> {
    ctx.say("Pong!").await?;
    Ok(())
}