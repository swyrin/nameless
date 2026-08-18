use std::path::PathBuf;

use serde::Deserialize;

use crate::config::sections::nameless_app::NamelessAppConfig;
use crate::utils::fs::{exists_on_fs, read_from_file};

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

        let config_path = &PathBuf::from(file);

        if exists_on_fs(config_path) {
            let content = read_from_file(config_path);
            let x: AppConfig = toml::from_str(&content).expect("Unable to parse string config.");

            x
        } else {
            tracing::warn!("Warning: missing config file.");
            tracing::warn!("Warning: The application should panic soon due to improper token.");

            Self::default()
        }
    }
}
