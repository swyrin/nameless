use poise::serenity_prelude::Message;
use viphurit::convert::make_viphurit;

pub async fn handle(message: &Message, ctx: &poise::serenity_prelude::Context) {
    let original_message = message.content.clone();
    let fixed_message = make_viphurit(original_message.clone()).to_string();
    let has_replacements = original_message != fixed_message;

    if has_replacements {
        let fixed_message = format!(
            r"
:x: {orig}
:white_check_mark: {fixed}
        ",
            orig = original_message,
            fixed = fixed_message
        );

        message.channel_id.say(&ctx, fixed_message).await.unwrap();
    }
}
