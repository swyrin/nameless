mod commands;
mod config;
mod handlers;
mod nameless_types;
mod persistence;
mod utils;

use crate::nameless_types::NamelessGlobalData;
use crate::persistence::connection::{acquire_database_connection, perform_database_migration};
use config::AppConfig;
use dotenvy::dotenv;
use handlers::event_handler;
use poise::serenity_prelude;
use tracing_subscriber::filter::EnvFilter;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let env_filter = EnvFilter::from_default_env().add_directive(
        "sqlx::query=DEBUG"
            .parse()
            .expect("Invalid tracing filter directive"),
    );

    tracing_subscriber::fmt()
        .with_test_writer()
        .with_env_filter(env_filter)
        .init();

    let pool = acquire_database_connection()
        .await
        .expect("Unable to acquire database connection.");

    perform_database_migration(&pool).await;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![commands::honeypot::honeypot(), commands::xkcd::xkcd()],
            event_handler: |ctx, event, fx, data| {
                Box::pin(event_handler::event_handler(ctx, event, fx, data))
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            let config = AppConfig::load();

            Box::pin(async move {
                match config.test_server_id {
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

    let config = AppConfig::load();

    let client = serenity_prelude::ClientBuilder::new(
        config.token,
        serenity_prelude::GatewayIntents::non_privileged()
            | serenity_prelude::GatewayIntents::MESSAGE_CONTENT,
    )
    .framework(framework)
    .await;

    client
        .unwrap()
        .start()
        .await
        .expect("Unable to start Discord bot.");
}
