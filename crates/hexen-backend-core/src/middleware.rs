use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use std::time::Instant;
use tracing::Span;

pub fn build_layer() -> impl tower::Layer<axum::Router> {
    tower::ServiceBuilder::new()
        .layer(axum::middleware::from_fn(trace_requests))
}

async fn trace_requests(req: Request<Body>, next: Next) -> Result<Response, StatusCode> {
    let start = Instant::now();
    let span = Span::current();
    let path = req.uri().path().to_string();
    let res = next.run(req).await;
    let status = res.status();
    let elapsed = start.elapsed();
    span.record("path", &tracing::field::display(&path));
    tracing::info!(%path, ?status, ?elapsed, "request completed");
    Ok(res)
}
