// @generated automatically by Diesel CLI.

diesel::table! {
    channel (id) {
        id -> Text,
        guild_id -> Text,
    }
}

diesel::table! {
    guild (id) {
        id -> Text,
    }
}

diesel::table! {
    honeypot (guild_id, channel_id) {
        guild_id -> Text,
        channel_id -> Text,
        enabled -> Bool,
    }
}

diesel::joinable!(channel -> guild (guild_id));
diesel::joinable!(honeypot -> channel (channel_id));
diesel::joinable!(honeypot -> guild (guild_id));

diesel::allow_tables_to_appear_in_same_query!(channel, guild, honeypot,);
