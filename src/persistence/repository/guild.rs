use crate::nameless_types::NamelessConnection;
use poise::serenity_prelude::GuildId;

/// Ensure guild record exists. Yes, upsert at home.
pub async fn ensure_exist_guild(guild_id: GuildId, connection: NamelessConnection) {
    sqlx::query!(
        "INSERT INTO guild (id) VALUES ($1) ON CONFLICT DO NOTHING;",
        guild_id.to_string()
    )
    .execute(&connection)
    .await
    .expect("Unable to insert guild entry");
}
