mod data;
mod logic;
mod presentation;

use std::{future::ready, net::SocketAddr};

use anyhow::Context;
use axum::{routing::get, Router};
use sqlx::{migrate, SqlitePool};
use tower_http::services::ServeDir;

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

    let app = Router::new()
        .route("/", get(presentation::pages::root))
        .route("/feed/:id/status", get(presentation::pages::feed_status))
        .route("/feed/:id/edit", get(presentation::pages::feed_edit))
        .route("/export/:code", get(logic::export))
        .fallback_service(
            ServeDir::new("assets")
                .not_found_service(get(|| ready(presentation::error::make_404()))),
        )
        .with_state(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    log::info!("listening on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .context("Error binding server")?;
    Ok(())
}
