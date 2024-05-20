use std::env;
use config::{Config, ConfigError, File, FileFormat};
use crate::infra::config::Settings;
use std::str::FromStr;
use env_logger::{Builder, WriteStyle};
use log::LevelFilter;

const ENV_KEY: &str = "FAMILY_API_ENV";

#[derive(Clone)]
pub struct Initializer {
    config_file_path: String,
    settings: Settings,
}

impl Default for Initializer {
    fn default() -> Self {
        let project_env = env::var(ENV_KEY).unwrap_or_else(|_| "DEV".to_string());

        let config_path = match project_env.as_str() {
            "TEST" => String::from("config/test.yaml"),
            "PROD" => String::from("config/prod.yaml"),
            _ => String::from("config/dev.yaml"),
        };

        Initializer {
            config_file_path: config_path,
            settings: Settings::default(),
        }
    }
}

impl Initializer {
    #[allow(dead_code)]
    pub fn new(config_file_path: String, settings: Settings) -> Self {
        Initializer {
            config_file_path,
            settings,
        }
    }

    pub fn settings(&self) -> &Settings {
        &self.settings
    }


    pub fn init_settings(mut self) -> Result<Initializer, ConfigError> {
        let settings = Config::builder()
            .set_default("default", "1")?
            .add_source(File::new(self.config_file_path.as_str(), FileFormat::Yaml))
            .set_override("override", "1")?
            .build()?;

        self.settings = settings.try_deserialize::<Settings>()?;

        Ok(self)
    }

    fn init_logger(self) -> Self {
        let mut builder = Builder::from_default_env();

        builder.filter(None, LevelFilter::from_str(&self.settings.log.level).unwrap_or(LevelFilter::Info));

        let color_mode = match self.settings.log.color_mode.as_str() {
            "always" => { WriteStyle::Always }
            "never" => { WriteStyle::Never }
            _ => { WriteStyle::Auto }
        };

        builder.write_style(color_mode);
        builder.init();

        self
    }

    pub fn must_init(self) -> Result<Self, ConfigError> {
        let ini = self.init_settings()?
            .init_logger();

        Ok(ini)
    }
}

#[cfg(test)]
mod tests {
    use super::Initializer;

    #[test]
    #[ignore]
    fn init_settings_with_output() -> Result<(), String> {
        let initializer = Initializer::default()
            .must_init()
            .unwrap();

        let settings = initializer.settings();

        if settings.ip != "127.0.0.1".to_string() {
            let err_msg = format!("ip is {}", settings.ip);
            return Err(err_msg);
        }
        if settings.port != "8000".to_string() {
            let err_msg = format!("port is {}", settings.port);
            return Err(err_msg);
        }
        Ok(())
    }

    // ENV=PROD cargo test prod
    #[test]
    #[should_panic]
    #[ignore]
    fn prod_config_file_not_exist() {
        let initializer = Initializer::default();
        let _settings = initializer.init_settings().unwrap();
    }

    #[test]
    fn init_settings() {
        let initializer = Initializer::default()
            .must_init()
            .unwrap();

        let settings = initializer.settings();

        assert_eq!(settings.ip, "127.0.0.1".to_string(), "ip: {}", settings.ip);
        assert_eq!(settings.port, "8000".to_string(), "port: {}", settings.port);
        // assert_eq!(settings.pg.host, Some("127.0.0.1".to_owned()));
        assert_eq!(settings.pg.port, Some(5432));
        assert_eq!(settings.pg.user, Some("postgres".to_owned()));
        assert_eq!(settings.pg.dbname, Some("postgres".to_owned()));
        assert_eq!(settings.log.level, "debug".to_owned());
        assert_eq!(settings.log.color_mode, "always".to_owned())
    }
}
