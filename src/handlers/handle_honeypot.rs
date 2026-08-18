use crate::nameless_types::NamelessGlobalData;
use crate::persistence::model::honeypot::Honeypot;
use crate::persistence::repository::honeypot::get_honeypot_entry;
use poise::serenity_prelude::{ChannelId, Message};
use std::str::FromStr;

pub async fn handle(
    message: &Message,
    ctx: &poise::serenity_prelude::Context,
    data: &NamelessGlobalData,
) {
    let mut db = &data.db;

    if let Some(gid) = message.guild_id
        && let Some(Honeypot {
            guild_id: _,
            channel_id,
            enabled,
        }) = get_honeypot_entry(gid, &mut db).await
    {
        let current_channel = message.channel_id;
        let target_channel = ChannelId::from_str(channel_id.as_str()).unwrap();

        if enabled && current_channel == target_channel {
            gid.ban(&ctx, message.author.id, 7)
                .await
                .expect("Probably a privileged member.");
        }
    }
}
