use crate::{Context, Error};

/// Add project to list
#[poise::command(prefix_command, slash_command)]
pub async fn remove_from_list(
    ctx: Context<'_>,
    #[description = "Specify the project name to add"] entry: Option<String>,
) -> Result<(), Error> {
    todo!("Implement");
    Ok(())
}
