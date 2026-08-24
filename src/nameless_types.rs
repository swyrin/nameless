use sqlx::{Pool, Sqlite};

pub type NamelessConnection = Pool<Sqlite>;

pub struct NamelessGlobalData {
    pub sql: NamelessConnection,
}

pub type NamelessError = Box<dyn std::error::Error + Send + Sync>;

pub type NamelessContext<'a> = poise::Context<'a, NamelessGlobalData, NamelessError>;
