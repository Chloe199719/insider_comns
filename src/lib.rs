use diesel_async::RunQueryDsl;
use tracing::info;

use crate::{
    app::{router::create_router, state::AppState},
    models::users::User,
    telemetry::setup_tracing,
};

pub mod app;
pub mod auth;
mod config;
pub mod db;
mod errors;
pub mod models;
pub mod schema;
mod telemetry;
pub async fn run() -> anyhow::Result<()> {
    setup_tracing();
    let settings = config::Settings::new()?;
    info!(target: "config", "Loaded settings");
    let pool_options = db::PoolOptions::default();
    let pool = db::create_pool(&settings.database.database_url, pool_options).await?;
    let redis_client = redis::Client::open(settings.database.redis_url.as_str())?;
    let redis = redis::aio::ConnectionManager::new(redis_client).await?;
    {
        let mut conn = redis.clone();
        let pong: String = redis::cmd("PING").query_async(&mut conn).await?;
        tracing::info!(%pong, "redis ready");
    }
    let listener = tokio::net::TcpListener::bind(&format!(
        "{}:{}",
        settings.application.host, settings.application.port
    ))
    .await?;
    tracing::info!(addr = %settings.application.host, "listening");
    axum::serve(
        listener,
        create_router(AppState::from_settings(&settings, redis, pool)).await,
    )
    .await?;
    // models::users::User::clean_up_users(&mut pool.get().await.unwrap()).await?;

    // let new_user = models::users::NewUser::new(
    //     "test@example.com".to_string(),
    //     auth::password::hash_password("password").unwrap(),
    // );
    // let user = new_user.insert_user(&mut pool.get().await.unwrap()).await?;
    // info!(target: "user", "User created: {:#?}", user);

    // let user = models::users::User::get_user_by_email("test@example.com")
    //     .first::<User>(&mut pool.get().await.unwrap())
    //     .await?;
    // info!(target: "user", "User found: {:#?}", user);
    // let user = models::users::User::get_user_by_id(&user.id)
    //     .first::<User>(&mut pool.get().await.unwrap())
    //     .await?;
    // info!(target: "user", "User found: {:#?}", user);
    Ok(())
}
