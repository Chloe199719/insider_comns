use std::env;

use serde::Deserialize;

use crate::errors::config_error;

#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
    pub environment: Environment,
    pub application: ApplicationSettings,
    pub database: DatabaseSettings,
    /// From `JWT_SECRET` only (not `config/app.yaml`).
    pub jwt_secret: String,
}
#[derive(Debug, Clone, Deserialize)]

pub struct DatabaseSettings {
    pub database_url: String,
    pub redis_url: String,
}
#[derive(Debug, Clone, Deserialize)]
pub struct ApplicationSettings {
    pub host: String,
    pub port: u16,
    pub base_url: String,

    pub jwt_access_expiry_secs: i64,
    pub jwt_refresh_expiry_secs: i64,
    pub refresh_cookie_name: String,
    pub refresh_cookie_path: String,
    pub refresh_cookie_secure: bool,
    pub refresh_same_site: RefreshSameSite,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize)]
pub enum RefreshSameSite {
    Strict,
    Lax,
    None,
}
impl RefreshSameSite {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Strict => "Strict",
            Self::Lax => "Lax",
            Self::None => "None",
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize)]
pub enum Environment {
    Development,
    Production,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Development => "development",
            Self::Production => "production",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "development" => Ok(Self::Development),
            "production" => Ok(Self::Production),
            other => Err(format!(
                "{} is not a supported environment. Use either `development` or `production`.",
                other
            )),
        }
    }
}
impl Settings {
    pub fn new() -> Result<Self, config_error::ConfigError> {
        dotenvy::dotenv().ok();
        let base_path = std::env::current_dir().expect("Failed to determine the current directory");
        let settings_directory = base_path.join("config");

        let settings = config::Config::builder()
            .add_source(config::File::from(settings_directory.join("app.yaml")))
            .build()?;
        let app_settings = settings.try_deserialize::<ApplicationSettings>()?;
        let environment: Environment = std::env::var("APP_ENVIRONMENT")
            .unwrap_or_else(|_| "development".into())
            .try_into()
            .expect("Failed to parse APP_ENVIRONMENT");

        let database_url = env::var("DATABASE_URL").map_err(|_| {
            config_error::ConfigError::EnvironmentError(std::env::VarError::NotPresent)
        })?;
        let redis_url =
            env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string());
        let jwt_secret = env::var("JWT_SECRET").map_err(|_| {
            config_error::ConfigError::EnvironmentError(std::env::VarError::NotPresent)
        })?;
        let settings = Self {
            environment,
            application: app_settings,
            database: DatabaseSettings {
                database_url,
                redis_url,
            },
            jwt_secret,
        };
        Ok(settings)
    }
}
