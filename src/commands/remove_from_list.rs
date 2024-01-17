use crate::{Context, Error};

/// Add project to list
#[poise::command(prefix_command, slash_command)]
pub async fn remove_from_list(
    ctx: Context<'_>,
    #[description = "Specify the project name to remove"] entry: Option<String>,
) -> Result<(), Error> {
    let mut projects = ctx.data().projects.lock().await;
    if entry.is_none() {
        ctx.say("No entry specified!").await?;
        return Ok(());
    }
    let index = projects.iter().position(|e| e == entry.as_ref().unwrap());
    if index.is_none() {
        ctx.say("Could not find entry!").await?;
        return Ok(());
    }
    ctx.say(format! {"Removed `{}` from projects list!", projects[index.unwrap()]})
        .await?;
    projects.remove(index.unwrap());
    Ok(())
}
