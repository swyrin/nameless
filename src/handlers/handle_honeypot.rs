use crate::nameless_types::NamelessGlobalData;
use crate::persistence::services::honeypot::get_honeypot_channel_id;
use poise::serenity_prelude::{ChannelId, Message};
use std::str::FromStr;

pub async fn handle(
    message: &Message,
    ctx: &poise::serenity_prelude::Context,
    data: &NamelessGlobalData,
) {
    let mut db = &data.db;

    if let Some(gid) = message.guild_id
        && let Some(cid) = get_honeypot_channel_id(gid, &mut db).await
    {
        let current_channel = message.channel_id;
        let target_channel = ChannelId::from_str(cid.as_str()).unwrap();

        if current_channel == target_channel {
            gid.ban(&ctx, message.author.id, 7)
                .await
                .expect("Probably a privileged member.");
        }
    }
}
