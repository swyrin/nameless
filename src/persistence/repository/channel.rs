use crate::nameless_types::NamelessConnection;
use poise::serenity_prelude::{ChannelId, GuildId};

/// Ensure channel record exists. Yes, upsert at home.
pub async fn ensure_exist_channel(
    guild_id: GuildId,
    channel_id: ChannelId,
    connection: NamelessConnection,
) {
    sqlx::query!(
        "INSERT INTO channel (id, guild_id) VALUES ($1, $2) ON CONFLICT DO NOTHING;",
        channel_id.to_string(),
        guild_id.to_string()
    )
    .execute(&connection)
    .await
    .expect("Unable to insert channel entry.");
}
