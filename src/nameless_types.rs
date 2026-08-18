use diesel_async::AsyncPgConnection;

pub type NamelessConnection = AsyncPgConnection;

pub struct NamelessGlobalData {
    pub db: NamelessConnection,
}

pub type NamelessError = Box<dyn std::error::Error + Send + Sync>;

pub type NamelessContext<'a> = poise::Context<'a, NamelessGlobalData, NamelessError>;
