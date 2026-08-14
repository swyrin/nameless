use std::path::PathBuf;

use serde::Deserialize;

use crate::config::sections::nameless_app::NamelessAppConfig;
use crate::utils::fs::read_from_file;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct AppConfig {
    nameless: NamelessAppConfig,
}

impl AppConfig {
    pub fn get_database_url(&self) -> String {
        self.nameless.database_url.clone()
    }

    pub fn get_token(&self) -> String {
        self.nameless.token.clone()
    }

    pub fn get_test_guild_id(self) -> Option<u64> {
        self.nameless.test_guild_id
    }

    /// Load `NAMELESS_CONFIG_FILE` or `nameless.toml`, whichever wins.
    pub fn load() -> Self {
        let file = std::env::var("NAMELESS_CONFIG_FILE").unwrap_or(String::from("nameless.toml"));
        let content = read_from_file(&PathBuf::from(file));

        let x: AppConfig = toml::from_str(&content).expect("Unable to parse string config.");

        x
    }
}
