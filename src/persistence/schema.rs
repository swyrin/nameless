// @generated automatically by Diesel CLI.

diesel::table! {
    guild (id) {
        id -> Text,
        honeypot_channel_id -> Nullable<Text>,
    }
}
