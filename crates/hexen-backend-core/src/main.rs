use axum::{routing::get, Router};
use tracing_subscriber::EnvFilter;

mod config;
mod security;
mod middleware;
mod approval_api;
mod apps_api;
mod xbox_api;
mod dev_endpoints;
mod suggestion_engine;
mod action_executor;
mod resilient_executor;
mod security_diagnostics;
mod security_upgrades;
mod fallback_mechanisms;
mod circuit_breaker;
mod health_checks;
mod emergency_system;
mod metrics;
mod game_metrics;
mod storage;
mod validation;
mod models;
mod csp_hardening;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let app = Router::new()
        .route("/health", get(health_checks::liveness))
        .route("/ready", get(health_checks::readiness))
        .merge(approval_api::router())
        .merge(apps_api::router())
        .merge(xbox_api::router())
        .layer(middleware::build_layer());

    let addr = config::listen_addr();
    tracing::info!("starting hexen-backend-core on {addr}");
    axum::Server::bind(&addr).serve(app.into_make_service()).await?;

    Ok(())
}
