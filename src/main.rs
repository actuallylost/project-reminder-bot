mod commands;
mod prisma;

use anyhow::Error;
use dotenv::dotenv;
use poise::serenity_prelude as serenity;
use poise::FrameworkError::*;
use std::env;
use tokio::sync::Mutex;
use tracing::{error, info};

use commands::add_to_list as add;
use commands::clear_projects as clear;
use commands::ping;
use commands::projects;
use commands::remove_from_list as remove;

// User data
struct Data {
    projects: Mutex<Vec<String>>,
}

type Context<'a> = poise::Context<'a, Data, Error>;

// Error handler
async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    match error {
        Setup { error, .. } => panic!("Failed to start bot: {:?}", error),
        Command { error, ctx, .. } => {
            error!("Error in command `{}`: {:?}", ctx.command().name, error);
        }
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                error!("Error while handling error: {:?}", e)
            }
        }
    }
}

#[tokio::main]
async fn main() {
    // Loads env file
    dotenv().ok();

    tracing_subscriber::fmt::init();

    // Defines token
    let token = env::var("DISCORD_TOKEN").expect("Missing DISCORD_TOKEN, check .env!");

    // Defines intents
    let intents =
        serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;

    // Defines framework options
    let framework = poise::Framework::builder()
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                info!("Logged in as {}", _ready.user.name);
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    projects: Mutex::new(Vec::new()),
                })
            })
        })
        .token(token)
        .intents(intents)
        .options(poise::FrameworkOptions {
            commands: vec![
                ping::ping(),
                projects::projects(),
                add::add_to_list(),
                remove::remove_from_list(),
                clear::clear_projects(),
            ],
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("!".into()),
                ..Default::default()
            },
            on_error: |error| Box::pin(on_error(error)),
            pre_command: |ctx| {
                Box::pin(async move {
                    info!(
                        "[LOG] Executing command {}...",
                        ctx.command().qualified_name
                    );
                })
            },
            // This code is run after a command if it was successful (returned Ok)
            post_command: |ctx| {
                Box::pin(async move {
                    info!("[LOG] Executed command {}!", ctx.command().qualified_name);
                })
            },
            ..Default::default()
        });

    framework.run().await.unwrap();
}
