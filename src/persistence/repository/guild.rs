use crate::nameless_types::NamelessConnection;
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
