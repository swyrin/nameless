use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "guild")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: String,

    #[sea_orm(unique)]
    pub honeypot_channel: Option<String>,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(DeriveIntoActiveModel)]
#[sea_orm(active_model = "crate::persistence::model::guild::ActiveModel")]
pub struct GuildHoneypotUpdatePayload {
    #[sea_orm(default = Some(""))]
    pub honeypot_channel: Option<Option<String>>,
}
