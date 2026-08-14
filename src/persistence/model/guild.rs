use diesel::prelude::*;

#[derive(Queryable, QueryableByName, Selectable, Identifiable)]
#[diesel(table_name = crate::persistence::schema::guild)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Guild {
    pub id: String,
    pub honeypot_channel_id: Option<String>,
}

#[derive(AsChangeset)]
#[diesel(table_name = crate::persistence::schema::guild)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct GuildUpdate {
    pub honeypot_channel_id: Option<String>,
}
