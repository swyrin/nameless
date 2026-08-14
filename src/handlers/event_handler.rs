use crate::handlers::{handle_honeypot, handle_viphurit};
use crate::nameless_types::{NamelessError, NamelessGlobalData};

pub async fn event_handler(
    ctx: &poise::serenity_prelude::Context,
    event: &poise::serenity_prelude::FullEvent,
    _framework: poise::FrameworkContext<'_, NamelessGlobalData, NamelessError>,
    data: &NamelessGlobalData,
) -> Result<(), NamelessError> {
    match event {
        poise::serenity_prelude::FullEvent::Ready { .. } => {
            tracing::info!("nameless* is ready!")
        }

        poise::serenity_prelude::FullEvent::Message { new_message } => {
            if new_message.author.id != ctx.cache.current_user().id {
                handle_honeypot::handle(new_message, ctx, data).await;
                handle_viphurit::handle(new_message, ctx).await;
            }
        }

        poise::serenity_prelude::FullEvent::Resume { .. } => {
            tracing::warn!("nameless* has just restarted!")
        }

        _ => {}
    }

    Ok(())
}
