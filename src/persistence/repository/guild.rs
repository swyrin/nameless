use crate::nameless_types::NamelessConnection;
use crate::persistence::model::guild::{Guild, GuildUpdate};
use crate::persistence::schema::guild;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use poise::serenity_prelude::GuildId;

pub async fn ensure_exist_guild(guild_id: GuildId, connection: &mut &NamelessConnection) {
    let gid = guild_id.to_string();

    diesel::insert_into(guild::table)
        .values(guild::id.eq(gid))
        .on_conflict_do_nothing()
        .execute(connection)
        .await
        .expect("Unable to upsert a guild entry.");
}

#[must_use = "Really?"]
pub async fn get_guild(guild_id: GuildId, connection: &mut &NamelessConnection) -> Guild {
    let gid = guild_id.to_string();

    let entries = guild::table
        .filter(guild::id.eq(gid))
        .limit(1)
        .load::<Guild>(connection)
        .await
        .expect("Unable to query for guild.");

    entries
        .into_iter()
        .next()
        .expect("Unable to retrieve guild record!")
}

pub async fn update_guild(
    guild_id: GuildId,
    new_guild_update: GuildUpdate,
    connection: &mut &NamelessConnection,
) -> Guild {
    let gid = guild_id.to_string();

    diesel::update(guild::table)
        .set(new_guild_update)
        .filter(guild::id.eq(gid))
        .get_result::<Guild>(connection)
        .await
        .expect("Unable to update a guild entry.")
}
