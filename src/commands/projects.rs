use crate::{Context, Error};

/// Show your project list
#[poise::command(prefix_command, slash_command, aliases("list"))]
pub async fn projects(ctx: Context<'_>) -> Result<(), Error> {
    let projects = ctx.data().projects.lock().await;
    if projects.is_empty() {
        ctx.say("Your project list is empty!").await?;
        return Ok(());
    }

    let mut response = String::from("Your projects are: ");
    for project in projects[0..projects.len() - 1].iter() {
        response += &format! {"`{}`, ", project};
    }
    response += &format! {" and `{}`.", projects[projects.len() - 1]};

    ctx.say(response).await?;
    Ok(())
}
