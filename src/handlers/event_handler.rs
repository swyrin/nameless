use poise::FrameworkContext;

use crate::handlers::{handle_honeypot, handle_viphurit};
use crate::types::{CommandData, CommandError};

pub async fn event_handler(
    framework: FrameworkContext<'_, CommandData, CommandError>,
    event: &poise::serenity_prelude::FullEvent,
) -> Result<(), CommandError> {
    let ctx = framework.serenity_context;

    match event {
        poise::serenity_prelude::FullEvent::Ready {
            ..
        } => {
            tracing::info!("nameless* is ready!");
        },

        poise::serenity_prelude::FullEvent::Message {
            new_message,
        } => {
            if new_message.author.id == ctx.cache.current_user().id {
                return Ok(());
            }

            handle_honeypot::handle(new_message, &framework).await;
            handle_viphurit::handle(new_message, &framework).await;
        },

        poise::serenity_prelude::FullEvent::Resume {
            ..
        } => {
            tracing::warn!("nameless* has just restarted!");
        },

        _ => {},
    }

    Ok(())
}
