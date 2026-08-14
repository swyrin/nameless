use crate::nameless_types::NamelessGlobalData;
use poise::serenity_prelude::Message;

pub async fn handle(message: &Message, ctx: &poise::serenity_prelude::Context) {
    let forbidden_knowledge = NamelessGlobalData::retrieve_forbidden_knowledge();

    let message_content = message.content.clone();
    let words: Vec<&str> = message_content.split_whitespace().collect();

    let words_fixed: Vec<&str> = words
        .iter()
        .map(|word| *forbidden_knowledge.get(word).unwrap_or(word))
        .collect();

    let fixed_message = words_fixed.join(" ");
    let has_replacements = message_content != fixed_message;

    if has_replacements {
        let fixed_message = format!(
            r"
:x: {orig}
:white_check_mark: {fixed}
        ",
            orig = message_content,
            fixed = fixed_message
        );

        message.channel_id.say(&ctx, fixed_message).await.unwrap();
    }
}
