use std::str::FromStr;

use crate::persistence::model::honeypot::Honeypot;
use crate::persistence::repository::channel::ensure_exist_channel;
use crate::{
    nameless_types::NamelessConnection, persistence::repository::guild::ensure_exist_guild,
};
use poise::serenity_prelude::{ChannelId, GuildId};
use sqlx::sqlite::SqliteQueryResult;

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
) -> SqliteQueryResult {
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
) -> SqliteQueryResult {
    sqlx::query!(
        "DELETE FROM honeypot WHERE guild_id = $1;",
        guild_id.to_string()
    )
    .execute(&connection)
    .await
    .expect("Unable to delete honeypot record.")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::nameless_types::NamelessConnection;
    use poise::serenity_prelude::GuildId;

    #[sqlx::test]
    async fn honeypot_nothing_burger(pool: NamelessConnection) {
        let mock_gid = GuildId::from(1);
        let entry = get_honeypot_entry(mock_gid, pool).await;

        assert!(entry.is_none());
    }

    #[sqlx::test]
    async fn honeypot_insertion_and_update(pool: NamelessConnection) {
        let mock_gid = GuildId::from(1);

        set_honeypot_entry(
            Honeypot {
                guild_id: "1".to_string(),
                channel_id: "1".to_string(),
                enabled: true,
            },
            pool.clone(),
        )
        .await;

        let entry = get_honeypot_entry(mock_gid, pool.clone()).await;

        match entry {
            Some(exists) => {
                assert_eq!(exists.clone().guild_id, "1");
                assert_eq!(exists.clone().channel_id, "1");
                assert!(exists.clone().enabled);
            }
            None => unreachable!(),
        }

        set_honeypot_entry(
            Honeypot {
                guild_id: "1".to_string(),
                channel_id: "2".to_string(),
                enabled: false,
            },
            pool.clone(),
        )
        .await;

        let entry = get_honeypot_entry(mock_gid, pool).await;

        match entry {
            Some(exists) => {
                assert_eq!(exists.clone().guild_id, "1");
                assert_eq!(exists.clone().channel_id, "2");
                assert!(!exists.clone().enabled);
            }
            None => unreachable!(),
        }
    }

    #[sqlx::test]
    async fn honeypot_deletion(pool: NamelessConnection) {
        let mock_gid = GuildId::from(1);

        set_honeypot_entry(
            Honeypot {
                guild_id: "1".to_string(),
                channel_id: "1".to_string(),
                enabled: true,
            },
            pool.clone(),
        )
        .await;

        let entry = get_honeypot_entry(mock_gid, pool.clone()).await;
        assert!(entry.is_some());

        delete_honeypot_entry(mock_gid, pool.clone()).await;

        let entry = get_honeypot_entry(mock_gid, pool).await;
        assert!(entry.is_none());
    }
}
