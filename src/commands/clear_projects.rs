use crate::{Context, Error};

/// Show your project list
#[poise::command(prefix_command, slash_command, aliases("clear_list"))]
pub async fn clear_projects(ctx: Context<'_>) -> Result<(), Error> {
    let mut projects = ctx.data().projects.lock().await;
    if projects.is_empty() {
        ctx.say("Your project list is empty!").await?;
        return Ok(());
    }

    let mut response = String::from("Your project list has been cleared! ");
    let count = projects.iter().fold(0, |acc, _x| acc + 1);
    projects.clear();
    response += &format!("(`{}` projects cleared)", count);

    ctx.say(response).await?;
    Ok(())
}
