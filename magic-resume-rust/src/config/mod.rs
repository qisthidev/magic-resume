use config::{Config, ConfigError, Environment, File};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIConfig {
    pub openai_api_key: Option<String>,
    pub openai_base_url: Option<String>,
    pub grammar_check_enabled: bool,
    pub polish_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub ai: AIConfig,
    pub static_files_dir: String,
    pub templates_dir: String,
    pub uploads_dir: String,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        let config = Config::builder()
            .add_source(File::with_name("config/default").required(false))
            .add_source(Environment::with_prefix("APP").separator("_"))
            .set_default("server.host", "0.0.0.0")?
            .set_default("server.port", 3000)?
            .set_default("database.url", "sqlite://./magic_resume.db")?
            .set_default("database.max_connections", 10)?
            .set_default("ai.grammar_check_enabled", true)?
            .set_default("ai.polish_enabled", true)?
            .set_default("static_files_dir", "./static")?
            .set_default("templates_dir", "./templates")?
            .set_default("uploads_dir", "./uploads")?
            .build()?;

        config.try_deserialize()
    }
}