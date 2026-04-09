//! Frontend SSR `einvoice-web` basé sur `axum` + `askama`.
//!
//! Sert une page d'accueil statique qui affichera plus tard la liste des
//! factures, les dépôts Chorus Pro et les événements.

use std::net::SocketAddr;

use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    title: &'a str,
    version: &'a str,
}

async fn index() -> impl IntoResponse {
    let tpl = IndexTemplate {
        title: "einvoice-rs",
        version: env!("CARGO_PKG_VERSION"),
    };
    match tpl.render() {
        Ok(body) => Html(body).into_response(),
        Err(err) => {
            tracing::error!(?err, "template render failed");
            (StatusCode::INTERNAL_SERVER_ERROR, "render error").into_response()
        }
    }
}

async fn health() -> &'static str {
    "ok"
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("info,einvoice_web=debug")),
        )
        .init();

    let app = Router::new()
        .route("/", get(index))
        .route("/health", get(health))
        .layer(TraceLayer::new_for_http());

    let addr: SocketAddr = std::env::var("WEB_BIND")
        .unwrap_or_else(|_| "0.0.0.0:8080".into())
        .parse()?;

    tracing::info!("einvoice-web listening on http://{addr}");

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
