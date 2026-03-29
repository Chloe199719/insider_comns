use chrono::Duration;
use redis::aio::ConnectionManager;

use crate::{
    config::{RefreshSameSite, Settings},
    db::DbPool,
};

#[derive(Clone)]
pub struct AppState {
    pub pool: DbPool,
    /// Shared Redis connection (caching, rate limits, etc.). Not used for refresh tokens.
    pub redis: ConnectionManager,
    pub jwt_secret: Vec<u8>,
    pub jwt_access_expiry: Duration,
    pub jwt_refresh_expiry_secs: u64,
    pub refresh_cookie_name: String,
    pub refresh_cookie_path: String,
    pub refresh_cookie_secure: bool,
    pub refresh_same_site: RefreshSameSite,
}

impl AppState {
    pub fn from_settings(settings: &Settings, redis: ConnectionManager, pool: DbPool) -> Self {
        Self {
            pool,
            redis,
            jwt_secret: settings.jwt_secret.as_bytes().to_vec(),
            jwt_access_expiry: Duration::seconds(settings.application.jwt_access_expiry_secs),
            jwt_refresh_expiry_secs: settings.application.jwt_refresh_expiry_secs as u64,
            refresh_cookie_name: settings.application.refresh_cookie_name.clone(),
            refresh_cookie_path: settings.application.refresh_cookie_path.clone(),
            refresh_cookie_secure: settings.application.refresh_cookie_secure,
            refresh_same_site: settings.application.refresh_same_site,
        }
    }
}
