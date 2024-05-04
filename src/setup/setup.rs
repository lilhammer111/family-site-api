use std::str::FromStr;
use deadpool_postgres::{Config as PgConfig};
use dotenv::dotenv;
use config::{Environment, Config};
use env_logger::{Builder, WriteStyle};
use log::LevelFilter;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Setup {
    pub server_addr: String,
    pub pg: PgConfig,
    pub log: LogConfig,
}

#[derive(Debug, Deserialize)]
pub struct LogConfig {
    pub level: String,
    pub color_mode: String, // always auto never
}

impl Setup {
    pub fn default_init() -> Self {
        Setup::default()
    }

    pub fn init_logger(&mut self) ->&mut Self {
        let mut builder = Builder::from_default_env();

        builder.filter(None, LevelFilter::from_str(&self.log.level).unwrap_or(LevelFilter::Info));

        let color_mode = match self.log.color_mode.as_str() {
            "always" => { WriteStyle::Always }
            "never" => { WriteStyle::Never }
            _ => { WriteStyle::Auto }
        };

        builder.write_style(color_mode);
        builder.init();

        self
    }

    pub fn init_pg(&mut self) ->&mut Self {
        println!("init pg");
        self
    }
}

impl Default for Setup {
    fn default() -> Self {
        dotenv().expect("Failed to load .env file");

        let config = Config::builder()
            .add_source(Environment::default())
            .build()
            .expect("Failed to build the config object");

        let setup: Setup = config.try_deserialize().expect("Failed to bind config to setup");

        setup
    }
}


#[cfg(test)]
mod tests {
    use crate::setup::setup::Setup;

    #[test]
    fn setup_works() {
        let setup = Setup::default();
        assert_eq!(setup.server_addr, "127.0.0.1:8080".to_owned());
        assert_eq!(setup.pg.host, Some("127.0.0.1".to_owned()));
        assert_eq!(setup.pg.port, Some(5432));
        assert_eq!(setup.pg.password, Some("puzhen0228".to_owned()));
        assert_eq!(setup.pg.dbname, Some("hammer".to_owned()));
        assert_eq!(setup.pg.pool.unwrap().max_size, 16);
        assert_eq!(setup.log.level, "info".to_owned());
        assert_eq!(setup.log.color_mode, "always".to_owned())
    }
}