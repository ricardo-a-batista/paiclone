use axum::{routing::get, Router};
use paiclone::{controller::home, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let app = Router::new().route("/", get(home::index));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}
