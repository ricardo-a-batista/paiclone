use axum::{routing::get, Router};
use paiclone::{controller::home, Result};
use tower::ServiceBuilder;
use tower_http::{
    compression::CompressionLayer, decompression::RequestDecompressionLayer, services::ServeDir,
};

#[tokio::main]
async fn main() -> Result<()> {
    tokio::try_join!(webserver())?;

    Ok(())
}

async fn webserver() -> Result<()> {
    let app = Router::new()
        .route("/", get(home::index))
        .nest_service("/statics", ServeDir::new("statics"))
        .layer(
            ServiceBuilder::new()
                .layer(RequestDecompressionLayer::new())
                .layer(CompressionLayer::new()),
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}
