use sqlx::{Pool, Postgres};

pub type NamelessConnection = Pool<Postgres>;

pub struct NamelessGlobalData {
    pub sql: NamelessConnection,
}

pub type NamelessError = Box<dyn std::error::Error + Send + Sync>;

pub type NamelessContext<'a> = poise::Context<'a, NamelessGlobalData, NamelessError>;
