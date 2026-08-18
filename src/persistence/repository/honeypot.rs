use std::str::FromStr;

use crate::persistence::model::honeypot::{Honeypot, HoneypotUpdate};
use crate::persistence::repository::channel::ensure_exist_channel;
use crate::persistence::schema::honeypot;
use crate::{
    nameless_types::NamelessConnection, persistence::repository::guild::ensure_exist_guild,
};
use diesel::query_dsl::methods::FilterDsl;
use diesel::{ExpressionMethods, OptionalExtension};
use diesel_async::RunQueryDsl;
use poise::serenity_prelude::{ChannelId, GuildId};

pub async fn get_honeypot_entry(
    guild_id: GuildId,
    connection: &mut &NamelessConnection,
) -> Option<Honeypot> {
    ensure_exist_guild(guild_id, connection).await;

    let gid = guild_id.to_string();

    let entry = honeypot::table
        .filter(honeypot::guild_id.eq(gid))
        .first::<Honeypot>(connection)
        .await
        .optional()
        .expect("Unable to query a honey entry.");

    entry
}

pub async fn set_honeypot_entry(
    new_honeypot_update: HoneypotUpdate,
    connection: &mut &NamelessConnection,
) {
    let HoneypotUpdate {
        guild_id,
        channel_id,
        enabled,
    } = new_honeypot_update;

    let guild_id = GuildId::from_str(&guild_id).unwrap();
    let channel_id = ChannelId::from_str(&channel_id).unwrap();

    ensure_exist_guild(guild_id, connection).await;
    ensure_exist_channel(guild_id, channel_id, connection).await;

    let gid = guild_id.to_string();
    let cid = channel_id.to_string();

    diesel::insert_into(honeypot::table)
        .values((
            honeypot::guild_id.eq(gid),
            honeypot::channel_id.eq(cid.clone()),
            honeypot::enabled.eq(enabled),
        ))
        .on_conflict(honeypot::guild_id)
        .do_update()
        .set((
            honeypot::channel_id.eq(cid.clone()),
            honeypot::enabled.eq(enabled),
        ))
        .execute(connection)
        .await
        .expect("Unable to update a honeypot entry.");
}

pub async fn delete_honeypot_entry(guild_id: GuildId, connection: &mut &NamelessConnection) {
    ensure_exist_guild(guild_id, connection).await;

    let gid = guild_id.to_string();

    let entry = honeypot::table.filter(honeypot::guild_id.eq(gid));

    diesel::delete(entry).execute(connection).await.unwrap();
}
