use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[serde(default)]
pub struct NamelessAppConfig {
    pub database_url: String,
    pub token: String,
    pub test_guild_id: Option<u64>,
}

impl Default for NamelessAppConfig {
    fn default() -> Self {
        Self {
            database_url: String::from("postgres://postgres:nameless@localhost:5432"),
            token: String::from(""),
            test_guild_id: None,
        }
    }
}
