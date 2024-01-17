use crate::{Context, Error};

/// Add project to list
#[poise::command(
    prefix_command,
    slash_command,
    aliases("update_list", "update_projects")
)]
pub async fn add_to_list(
    ctx: Context<'_>,
    #[description = "Specify the project name to add"] entry: Option<String>,
) -> Result<(), Error> {
    if entry.is_none() {
        ctx.say("No entry specified!").await?;
        return Ok(());
    }
    let mut projects = ctx.data().projects.lock().await;
    ctx.say(format!(
        "Added `{}` to the project list!",
        entry.as_ref().unwrap()
    ))
    .await?;
    projects.push(entry.unwrap());
    Ok(())
}
