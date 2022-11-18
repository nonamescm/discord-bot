use crate::{commands::general::ping::ping, primitives::State};
use anyhow::{Context, Result};
use dotenv::dotenv;
use poise::{
    samples::register_in_guild,
    serenity_prelude::{CacheHttp, GatewayIntents, GuildId},
    Framework, FrameworkOptions,
};
use std::{env, fs, path::Path, process};
use tracing::log::info;
use tracing_subscriber::EnvFilter;

mod commands;
mod primitives;

fn copy_dotenv() -> Result<()> {
    if !Path::new(".env").exists() {
        info!("Uh, I can't find `.env` file. So i'm copying `.env.example` to `.env`");
        fs::copy(".env.example", ".env").context("Failed to copy `.env` file")?;

        info!("Configure the `.env` then re-run the bot. Please.");
        process::exit(0)
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::from_default_env().add_directive("codify=debug".parse().unwrap()),
        )
        .init();

    copy_dotenv()?;
    dotenv().context("Failed to load `.env` file")?;

    info!("Starting bot...");
    let guild_id: u64 = env::var("CODIFY_GUILD_ID")
        .context("Failed to read $DISCORD_GUILD_ID")?
        .parse()
        .context("Failed to parse $DISCORD_GUILD_ID as a valid integer!")?;

    let commands = vec![ping()];

    let framework = Framework::builder()
        .token(env::var("DISCORD_TOKEN").context("Failed to read $DISCORD_TOKEN")?)
        .intents(GatewayIntents::privileged())
        .options(FrameworkOptions {
            commands,
            ..Default::default()
        })
        .setup(move |cx, _, f| {
            Box::pin(async move {
                register_in_guild(&cx.http(), &f.options().commands, GuildId(guild_id)).await?;
                Ok(State {})
            })
        });

    framework.run().await?;

    Ok(())
}
