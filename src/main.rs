mod commands;
mod config;
mod handlers;
mod nameless_types;
mod persistence;
mod utils;

use crate::persistence::connection::{acquire_database_connection, perform_database_migration};
use crate::{nameless_types::NamelessGlobalData, utils::fs::get_cwd};
use config::data::AppConfig;
use handlers::event_handler;
use poise::serenity_prelude;
use tracing_subscriber::filter::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let env_filter = EnvFilter::from_default_env().add_directive("sqlx::query=DEBUG".parse()?);

    tracing_subscriber::fmt()
        .with_test_writer()
        .with_env_filter(env_filter)
        .init();

    let config = AppConfig::load();

    let pool = acquire_database_connection().await?;
    perform_database_migration(&pool).await?;

    let token = config.get_token();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![commands::honeypot::honeypot(), commands::xkcd::xkcd()],
            event_handler: |ctx, event, fx, data| {
                Box::pin(event_handler::event_handler(ctx, event, fx, data))
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                match config.get_test_guild_id() {
                    Some(id) => {
                        tracing::warn!(
                            "{}",
                            format!("Command is registered locally in guild {}", id)
                        );

                        poise::builtins::register_in_guild(
                            ctx,
                            &framework.options().commands,
                            id.into(),
                        )
                        .await?
                    }
                    None => {
                        tracing::info!("Command is registered globally.");

                        poise::builtins::register_globally(ctx, &framework.options().commands)
                            .await?
                    }
                }

                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(NamelessGlobalData { sql: pool })
            })
        })
        .build();

    let client = serenity_prelude::ClientBuilder::new(
        token,
        serenity_prelude::GatewayIntents::non_privileged()
            | serenity_prelude::GatewayIntents::MESSAGE_CONTENT,
    )
    .framework(framework)
    .await;

    client?.start().await?;

    Ok(())
}
