use axum::{
    Router,
    body::Bytes,
    extract::{MatchedPath, Request},
    http::HeaderMap,
    response::Response,
    routing::get,
};
use std::time::Duration;

use tower_http::classify::ServerErrorsFailureClass;
use tracing::{Span, info_span, instrument};

use crate::app::state::AppState;

pub async fn create_router(state: AppState) -> Router {
    let router = Router::new()
        .route("/health", get(health_check))
        .layer(tower_http::cors::CorsLayer::permissive())
        .layer(
            tower_http::trace::TraceLayer::new_for_http()
                .make_span_with(|request: &Request<_>| {
                    // Log the matched route's path (with placeholders not filled in).
                    // Use request.uri() or OriginalUri if you want the real path.
                    let matched_path = request
                        .extensions()
                        .get::<MatchedPath>()
                        .map(MatchedPath::as_str);

                    info_span!(
                        "http_request",
                        method = ?request.method(),
                        matched_path,
                        some_other_field = tracing::field::Empty,
                    )
                })
                .on_request(|request: &Request<_>, _span: &Span| {
                    tracing::info!(
                        method = %request.method(),
                        uri = %request.uri(),
                        "request started"
                    );
                })
                .on_response(|response: &Response, latency: Duration, _span: &Span| {
                    tracing::info!(
                        status = %response.status(),
                        latency_ms = latency.as_millis(),
                        "response sent"
                    );
                })
                .on_body_chunk(|chunk: &Bytes, latency: Duration, _span: &Span| {
                    tracing::debug!(
                        chunk_len = chunk.len(),
                        latency_ms = latency.as_millis(),
                        "body chunk"
                    );
                })
                .on_eos(
                    |trailers: Option<&HeaderMap>, stream_duration: Duration, _span: &Span| {
                        tracing::info!(
                            has_trailers = trailers.is_some(),
                            stream_duration_ms = stream_duration.as_millis(),
                            "stream finished"
                        );
                    },
                )
                .on_failure(|error: ServerErrorsFailureClass, latency: Duration, _span: &Span| {
                    tracing::warn!(
                        ?error,
                        latency_ms = latency.as_millis(),
                        "request failed"
                    );
                }),
        )
        .with_state(state);
    router
}

#[instrument]
async fn health_check() -> &'static str {
    tracing::info!("health check");
    "OK"
}
