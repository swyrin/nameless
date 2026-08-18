use diesel::prelude::*;

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq, Clone)]
#[diesel(belongs_to(crate::persistence::model::guild::Guild))]
#[diesel(belongs_to(crate::persistence::model::channel::Channel))]
#[diesel(primary_key(guild_id, channel_id))]
#[diesel(table_name = crate::persistence::schema::honeypot)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Honeypot {
    pub guild_id: String,
    pub channel_id: String,
    pub enabled: bool,
}

#[derive(AsChangeset)]
#[diesel(table_name = crate::persistence::schema::honeypot)]
pub struct HoneypotUpdate {
    pub guild_id: String,
    pub channel_id: String,
    pub enabled: bool,
}
