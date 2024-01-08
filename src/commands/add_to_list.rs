use crate::{Context, Error};

/// Show your project list
#[poise::command(prefix_command, slash_command, aliases("add"))]
pub async fn add_to_list(
    ctx: Context<'_>,
) -> Result<(), Error> {
    todo!("Add to list commands");
    Ok(())
}