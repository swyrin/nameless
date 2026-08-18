use crate::nameless_types::NamelessConnection;
use crate::persistence::schema::channel;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use poise::serenity_prelude::{ChannelId, GuildId};

pub async fn ensure_exist_channel(
    guild_id: GuildId,
    channel_id: ChannelId,
    connection: &mut &NamelessConnection,
) {
    let gid = guild_id.to_string();
    let cid = channel_id.to_string();

    diesel::insert_into(channel::table)
        .values((channel::id.eq(cid), channel::guild_id.eq(gid)))
        .on_conflict_do_nothing()
        .execute(connection)
        .await
        .expect("Unable to upsert a channel entry.");
}
