use crate::nameless_types::NamelessConnection;
use crate::persistence::repository::guild::{ensure_exist_guild, update_guild};
use crate::persistence::{model::guild::GuildUpdate, repository::guild::get_guild};
use poise::serenity_prelude::{ChannelId, GuildId};

pub async fn get_honeypot_channel_id(
    guild_id: GuildId,
    connection: &mut &NamelessConnection,
) -> Option<String> {
    ensure_exist_guild(guild_id, connection).await;

    let guild = get_guild(guild_id, connection).await;

    guild.honeypot_channel_id
}

pub async fn set_honeypot_channel_id(
    guild_id: GuildId,
    channel_id: Option<ChannelId>,
    connection: &mut &NamelessConnection,
) {
    ensure_exist_guild(guild_id, connection).await;

    let changeset = match channel_id {
        Some(chn) => {
            let channel_id = chn.to_string();

            GuildUpdate {
                honeypot_channel_id: Some(channel_id),
            }
        }
        None => GuildUpdate {
            honeypot_channel_id: None,
        },
    };

    update_guild(guild_id, changeset, connection).await;
}
