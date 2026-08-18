use diesel::prelude::*;

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq)]
#[diesel(belongs_to(crate::persistence::model::guild::Guild))]
#[diesel(table_name = crate::persistence::schema::channel)]
pub struct Channel {
    pub id: String,
    pub guild_id: String,
}
