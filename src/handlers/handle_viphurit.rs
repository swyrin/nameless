use poise::serenity_prelude::Message;
use viphurit::convert::make_viphurit;

pub async fn handle(message: &Message, ctx: &poise::serenity_prelude::Context) {
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

        message.channel_id.say(&ctx, fixed_message).await.unwrap();
    }
}
