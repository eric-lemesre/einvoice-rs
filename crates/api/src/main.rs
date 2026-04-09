//! Point d'entrée de l'API REST `einvoice-api`.
//!
//! Pour l'instant l'API expose uniquement `/` et `/health`. La persistance
//! PostgreSQL (via `sqlx`) et les routes CRUD `/invoices/*` seront ajoutées
//! lorsque le sérialiseur Factur-X sera implémenté.

use std::net::SocketAddr;

use axum::{routing::get, Json, Router};
use serde_json::json;
use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("info,einvoice_api=debug")),
        )
        .init();

    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health))
        .layer(TraceLayer::new_for_http());

    let addr: SocketAddr = std::env::var("API_BIND")
        .unwrap_or_else(|_| "0.0.0.0:3000".into())
        .parse()?;

    tracing::info!("einvoice-api listening on http://{addr}");

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

async fn root() -> Json<serde_json::Value> {
    Json(json!({
        "name": "einvoice-rs",
        "service": "api",
        "version": env!("CARGO_PKG_VERSION"),
    }))
}

async fn health() -> Json<serde_json::Value> {
    Json(json!({ "status": "ok" }))
}
