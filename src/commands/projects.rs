use crate::{prisma::project, Context, Error};

/// Show your project list
#[poise::command(prefix_command, slash_command, aliases("list"))]
pub async fn projects(ctx: Context<'_>) -> Result<(), Error> {
    // Bot projects
    let mut projects = ctx.data().projects.lock().await;
    // Database projects
    let entries = ctx
        .data()
        .client
        .project()
        .find_many(vec![project::user_id::equals(Some(
            ctx.author().id.to_string().parse::<i64>().unwrap(),
        ))])
        .exec()
        .await
        .unwrap();
    // Check if database contains no project entries for the command author
    if entries.is_empty() {
        ctx.say("Your project list is empty!").await?;
        return Ok(());
    }
    let mut response = String::from("Your projects are: ");
    // If there's only one entry, push it to project cache and send response
    if entries.len() == 1 {
        response += &format!("`{:?}`", entries[0]);
        projects.push(entries[0].clone());
        ctx.say(response).await?;
        return Ok(());
    }
    // For every entry but the last one add a comma between them and push it to project cache
    for entry in entries[0..entries.len() - 1].iter() {
        response += &format!("`{:?}`, ", entry);
        projects.push(entry.clone());
    }
    // Do the same but for the last entry
    response += &format!(" and `{:?}`.", entries[entries.len() - 1]);
    projects.push(entries[entries.len() - 1].clone());

    ctx.say(response).await?;
    Ok(())
}
