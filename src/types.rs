use sea_orm::DatabaseConnection;

pub struct CommandData {
    pub db: DatabaseConnection,
}

pub type CommandError = Box<dyn std::error::Error + Send + Sync>;

pub type CommandContext<'ctx> = poise::Context<'ctx, CommandData, CommandError>;
