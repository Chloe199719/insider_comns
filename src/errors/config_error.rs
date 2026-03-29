#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Failed to read configuration file: {0}")]
    FileReadError(std::io::Error),
    #[error("Failed to parse configuration file: {0}")]
    FileParseError(config::ConfigError),

    #[error("Failed to load environment variables: {0}")]
    EnvironmentError(std::env::VarError),
}
impl From<std::io::Error> for ConfigError {
    fn from(error: std::io::Error) -> Self {
        Self::FileReadError(error)
    }
}
impl From<config::ConfigError> for ConfigError {
    fn from(error: config::ConfigError) -> Self {
        Self::FileParseError(error)
    }
}
