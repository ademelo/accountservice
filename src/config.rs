use config::{Config, ConfigError, File};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    pub host: String,
    pub user: String,
    pub password: String,
}

impl DatabaseConfig {
    pub fn load() -> Result<Self, ConfigError> {
        let settings = Config::builder()
            .add_source(File::with_name("config/dev.toml"))
            .build()?;
        settings.try_deserialize()
    }

    pub fn connection_string(&self) -> String {
        format!(
            "host={} user={} password={} dbname=postgres",
            self.host, self.user, self.password
        )
    }

    pub fn database_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:5432/postgres",
            self.user, self.password, self.host
        )
    }
}
