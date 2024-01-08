use crate::{Context, Error};

/// Show your project list
#[poise::command(prefix_command, slash_command, aliases("projects"))]
pub async fn list(
    ctx: Context<'_>,
) -> Result<(), Error> {
    ctx.say("Listing your projects!").await?;
    Ok(())
}