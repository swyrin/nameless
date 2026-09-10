use crate::types::{CommandData, CommandError};
use crate::utils::make_viphurit::make_viphurit;
use poise::FrameworkContext;
use poise::serenity_prelude::Message;

pub async fn handle(
    message: &Message,
    framework: &FrameworkContext<'_, CommandData, CommandError>,
) {
    let original_message = message.content.clone();
    let fixed_message = make_viphurit(&original_message.clone()).clone();
    let has_replacements = original_message != fixed_message;

    if has_replacements {
        let fixed_message = format!(
            r"
:x: {original_message}
:white_check_mark: {fixed_message}
        "
        );

        message
            .channel_id
            .say(&framework.serenity_context, fixed_message)
            .await
            .unwrap();
    }
}
