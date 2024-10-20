use axum::{routing::get, Router};
use paiclone::{controller::home, infra::state::State, Result};
use sqlx::{migrate::Migrator, sqlite::SqlitePoolOptions};
use tower::ServiceBuilder;
use tower_http::{
    compression::CompressionLayer, decompression::RequestDecompressionLayer, services::ServeDir,
};
use tracing::{info, instrument};

static MIGRATOR: Migrator = sqlx::migrate!();

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let pool = SqlitePoolOptions::new()
        .connect("./paiclone.db?mode=rwc")
        .await?;
    MIGRATOR.run(&pool).await?;

    let state = State::new(pool);
    tokio::try_join!(webserver(state))?;
    Ok(())
}

#[instrument(skip(state))]
async fn webserver(state: State) -> Result<()> {
    let app = Router::new()
        .route("/", get(home::index))
        .nest_service("/statics", ServeDir::new("statics"))
        .layer(
            ServiceBuilder::new()
                .layer(RequestDecompressionLayer::new())
                .layer(CompressionLayer::new()),
        )
        .with_state(state);

    info!("App Server starting on http://localhost:3000");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}
