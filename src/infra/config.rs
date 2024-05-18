use deadpool_postgres::{Config as PgConfig};
use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct Settings {
    pub ip: String,
    pub port: String,
    pub jwt_secret: String,
    pub static_file_path: String,
    pub pg: PgConfig,
    pub log: LogConfig,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct LogConfig {
    pub level: String,
    pub color_mode: String, // always auto never
}


impl Settings {}
