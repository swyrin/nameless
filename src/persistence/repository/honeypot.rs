use std::str::FromStr;

use crate::persistence::model::honeypot::Honeypot;
use crate::persistence::repository::channel::ensure_exist_channel;
use crate::{
    nameless_types::NamelessConnection, persistence::repository::guild::ensure_exist_guild,
};
use poise::serenity_prelude::{ChannelId, GuildId};
use sqlx::postgres::PgQueryResult;

/// Query honeypot entry.
pub async fn get_honeypot_entry(
    guild_id: GuildId,
    connection: NamelessConnection,
) -> Option<Honeypot> {
    ensure_exist_guild(guild_id, connection.clone()).await;

    let result = sqlx::query_as!(
        Honeypot,
        "SELECT * FROM honeypot WHERE guild_id = $1 LIMIT 1;",
        guild_id.to_string()
    )
    .fetch_optional(&connection)
    .await
    .expect("Unable to query a honeypot entry.");

    result
}

/// "Upsert" honeypot entry.
pub async fn set_honeypot_entry(
    new_honeypot_update: Honeypot,
    connection: NamelessConnection,
) -> PgQueryResult {
    let Honeypot {
        guild_id,
        channel_id,
        enabled,
    } = new_honeypot_update;

    let guild_id = GuildId::from_str(&guild_id).unwrap();
    let channel_id = ChannelId::from_str(&channel_id).unwrap();

    ensure_exist_guild(guild_id, connection.clone()).await;
    ensure_exist_channel(guild_id, channel_id, connection.clone()).await;

    let result = sqlx::query!(
        "INSERT INTO honeypot(guild_id, channel_id, enabled)
            VALUES ($1, $2, $3)
            ON CONFLICT (guild_id)
            DO UPDATE
            SET
                channel_id = $2,
                enabled = $3;",
        guild_id.to_string(),
        channel_id.to_string(),
        enabled
    )
    .execute(&connection)
    .await
    .expect("Unable to update a honeypot entry.");

    result
}

/// Delete honeypot entry.
pub async fn delete_honeypot_entry(
    guild_id: GuildId,
    connection: NamelessConnection,
) -> PgQueryResult {
    sqlx::query!(
        "DELETE FROM honeypot WHERE guild_id = $1;",
        guild_id.to_string()
    )
    .execute(&connection)
    .await
    .expect("Unable to delete honeypot record.")
}
