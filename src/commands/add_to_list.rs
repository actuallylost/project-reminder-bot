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
    let mut response = String::new();
    let mut projects = ctx.data().projects.lock().await;
    match &entry {
        Some(val) => {
            projects.push(val.clone());
            response += &format! {"Added `{}` to the project list!", val}
        }
        None => response += "No project added.",
    }

    ctx.say(response).await?;
    Ok(())
}
