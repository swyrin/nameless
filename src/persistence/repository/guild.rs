use poise::serenity_prelude::GuildId;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter};

use crate::persistence::model::guild;

/// Get guild record.
pub async fn get_guild(id: GuildId, connection: &DatabaseConnection) -> Option<guild::Model> {
    guild::Entity::find_by_id(id.to_string())
        .one(connection)
        .await
        .expect("Unable to perform guild retrieval.")
}

/// Insert guild record.
pub async fn insert_guild(
    data: guild::ActiveModel,
    connection: &DatabaseConnection,
) -> guild::Model {
    guild::Entity::insert(data)
        .exec_with_returning(connection)
        .await
        .expect("Unable to perform guild insertion.")
}

/// Update guild record.
pub async fn update_guild(
    id: GuildId,
    data: impl IntoActiveModel<guild::ActiveModel>,
    connection: &DatabaseConnection,
) -> Vec<guild::Model> {
    guild::Entity::update_many()
        .set(data.into_active_model())
        .filter(guild::Column::Id.eq(id.to_string()))
        .exec_with_returning(connection)
        .await
        .expect("Unable to perform guild update.")
}
