use std::time::Duration;

use diesel::{ConnectionError, ConnectionResult};
use diesel_async::AsyncPgConnection;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::pooled_connection::ManagerConfig;
use diesel_async::pooled_connection::bb8::Pool;
use futures_util::FutureExt;
use futures_util::future::BoxFuture;
use rustls::ClientConfig;
use rustls_platform_verifier::ConfigVerifierExt;

/// Async Diesel connection pool over PostgreSQL with TLS (`sslmode`-style via rustls).
pub type DbPool = Pool<AsyncPgConnection>;

/// Pool sizing and timeouts; tune per deployment.
#[derive(Debug, Clone)]
pub struct PoolOptions {
    pub max_size: u32,
    pub min_idle: Option<u32>,
    pub max_lifetime: Option<Duration>,
    pub idle_timeout: Option<Duration>,
}

impl Default for PoolOptions {
    fn default() -> Self {
        Self {
            max_size: 10,
            min_idle: Some(5),
            max_lifetime: Some(Duration::from_secs(60 * 60 * 24)),
            idle_timeout: Some(Duration::from_secs(60 * 2)),
        }
    }
}

/// Builds a bb8 pool using rustls with platform certificate verification (similar to `sslmode=verify-full`).
///
/// See [diesel_async pooled-with-rustls example](https://github.com/diesel-rs/diesel_async/blob/main/examples/postgres/pooled-with-rustls/src/main.rs).
pub async fn create_pool(
    database_url: impl Into<String>,
    options: PoolOptions,
) -> anyhow::Result<DbPool> {
    let database_url = database_url.into();
    let mut mgr_config = ManagerConfig::default();
    mgr_config.custom_setup = Box::new(establish_connection);

    let mgr = AsyncDieselConnectionManager::<AsyncPgConnection>::new_with_config(
        database_url,
        mgr_config,
    );

    let mut builder = Pool::builder().max_size(options.max_size);
    if let Some(n) = options.min_idle {
        builder = builder.min_idle(Some(n));
    }
    if let Some(d) = options.max_lifetime {
        builder = builder.max_lifetime(Some(d));
    }
    if let Some(d) = options.idle_timeout {
        builder = builder.idle_timeout(Some(d));
    }

    let pool = builder.build(mgr).await?;
    Ok(pool)
}

fn establish_connection(config: &str) -> BoxFuture<'_, ConnectionResult<AsyncPgConnection>> {
    let fut = async move {
        let rustls_config = ClientConfig::with_platform_verifier().map_err(|e| {
            ConnectionError::BadConnection(format!("rustls platform verifier: {e}"))
        })?;
        let tls = tokio_postgres_rustls::MakeRustlsConnect::new(rustls_config);
        let (client, conn) = tokio_postgres::connect(config, tls)
            .await
            .map_err(|e| ConnectionError::BadConnection(e.to_string()))?;

        AsyncPgConnection::try_from_client_and_connection(client, conn).await
    };
    fut.boxed()
}
