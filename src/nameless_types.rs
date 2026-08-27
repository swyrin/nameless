use sea_orm::DatabaseConnection;

pub struct NamelessGlobalData {
    pub sql: DatabaseConnection,
}

pub type NamelessError = Box<dyn std::error::Error + Send + Sync>;

pub type NamelessContext<'a> = poise::Context<'a, NamelessGlobalData, NamelessError>;
