use crate::persistence::repository::guild::get_guild;
use crate::types::{CommandData, CommandError};
use poise::FrameworkContext;
use poise::serenity_prelude::{ChannelId, Message};
use std::str::FromStr;

pub async fn handle(
    message: &Message,
    framework: &FrameworkContext<'_, CommandData, CommandError>,
) {
    let db = framework.user_data.db.clone();

    if let Some(gid) = message.guild_id
        && let Some(entry) = get_guild(gid, &db).await
    {
        let current_channel = message.channel_id;

        if let Some(honeypot_channel) = entry.honeypot_channel
            && current_channel == ChannelId::from_str(&honeypot_channel).unwrap()
        {
            gid.ban(&framework.serenity_context, message.author.id, 7)
                .await
                .expect("Probably a privileged member.");
        }
    }
}
