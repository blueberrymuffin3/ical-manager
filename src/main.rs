mod data;
mod logic;
mod presentation;
mod strum_util;

use std::{future::ready, net::SocketAddr};

use anyhow::Context;
use axum::{
    routing::{get, post},
    Router,
};
use presentation::auth::AuthManager;
use sqlx::{migrate, SqlitePool};
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;

use crate::{data::secrets::SecretReader, presentation::auth::LOCATION_LOGIN};

#[derive(axum::extract::FromRef, Clone)]
struct AppState {
    pool: SqlitePool,
    auth: AuthManager,
    cookie_key: tower_cookies::Key,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().context("Error loading .env file")?;
    env_logger::init();

    let database_url = std::env::var("DATABASE_URL").context("Error loading DATABASE_URL")?;
    let pool = SqlitePool::connect(&database_url)
        .await
        .context("Error connecting to database")?;

    migrate!()
        .run(&pool)
        .await
        .context("Error running migration")?;

    let auth = AuthManager::new()
        .await
        .context("Error making AuthManager")?;

    let cookie_key = SecretReader::read_or_gen(&pool).await?;

    // TODO: Save the key in the database
    let state: AppState = AppState {
        pool,
        auth,
        cookie_key,
    };

    let app = Router::new()
        // Pages
        .route("/", get(presentation::pages::root))
        .route("/feed/:id/status", get(presentation::pages::feed_status))
        .route("/feed/:id/edit", get(presentation::pages::feed_edit_get))
        .route("/feed/:id/edit", post(presentation::pages::feed_edit_post))
        .route("/feed/create", get(presentation::pages::feed_create_get))
        .route("/feed/create", post(presentation::pages::feed_create_post))
        .nest(LOCATION_LOGIN, presentation::auth::router())
        // API
        .route("/export/:code", get(presentation::pages::export))
        //
        .layer(CookieManagerLayer::new())
        .fallback_service(
            ServeDir::new("assets")
                .not_found_service(get(|| ready(presentation::error::make_404()))),
        )
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    log::info!("listening on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .context("Error binding server")?;
    Ok(())
}
