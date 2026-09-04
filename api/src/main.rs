mod csrf;
mod mailer;
mod model;
mod ratelimit;
mod routes;

use std::net::SocketAddr;
use std::sync::Arc;

use tracing_subscriber::EnvFilter;

use crate::mailer::Mailer;
use crate::ratelimit::RateLimiter;
use crate::routes::AppState;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    let mailer = Mailer::from_env().map(Arc::new);
    if mailer.is_none() {
        tracing::warn!("SMTP is not configured, POST /api/contact will return 503");
    }

    let state = AppState {
        projects: Arc::new(routes::ProjectsPayload::new(&model::load_projects())),
        mailer,
        limiter: Arc::new(RateLimiter::new()),
    };

    let port = std::env::var("PORT")
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(8080u16);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("failed to bind");
    tracing::info!("listening on {addr}");

    axum::serve(listener, routes::router(state))
        .with_graceful_shutdown(shutdown_signal())
        .await
        .expect("server error");
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("failed to listen for ctrl-c");
}
