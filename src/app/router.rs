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
                        request_uri = tracing::field::Empty,
                        response_status = tracing::field::Empty,
                        response_latency_ms = tracing::field::Empty,
                        last_chunk_len = tracing::field::Empty,
                        chunk_latency_ms = tracing::field::Empty,
                        stream_duration_ms = tracing::field::Empty,
                        has_trailers = tracing::field::Empty,
                        failure_class = tracing::field::Empty,
                        failure_latency_ms = tracing::field::Empty,
                    )
                })
                .on_request(|request: &Request<_>, span: &Span| {
                    span.record("request_uri", tracing::field::display(request.uri()));
                    tracing::info!(
                        method = %request.method(),
                        uri = %request.uri(),
                        "request started"
                    );
                })
                .on_response(|response: &Response, latency: Duration, span: &Span| {
                    let latency_ms: u64 = latency.as_millis().try_into().unwrap_or(u64::MAX);
                    span.record(
                        "response_status",
                        tracing::field::display(response.status()),
                    );
                    span.record("response_latency_ms", latency_ms);
                    tracing::info!(
                        status = %response.status(),
                        latency_ms,
                        "response sent"
                    );
                })
                .on_body_chunk(|chunk: &Bytes, latency: Duration, span: &Span| {
                    let latency_ms: u64 = latency.as_millis().try_into().unwrap_or(u64::MAX);
                    span.record("last_chunk_len", chunk.len() as u64);
                    span.record("chunk_latency_ms", latency_ms);
                    tracing::debug!(
                        chunk_len = chunk.len(),
                        latency_ms,
                        "body chunk"
                    );
                })
                .on_eos(
                    |trailers: Option<&HeaderMap>, stream_duration: Duration, span: &Span| {
                        let duration_ms: u64 = stream_duration
                            .as_millis()
                            .try_into()
                            .unwrap_or(u64::MAX);
                        span.record("stream_duration_ms", duration_ms);
                        span.record("has_trailers", trailers.is_some());
                        tracing::info!(
                            has_trailers = trailers.is_some(),
                            stream_duration_ms = duration_ms,
                            "stream finished"
                        );
                    },
                )
                .on_failure(|error: ServerErrorsFailureClass, latency: Duration, span: &Span| {
                    let latency_ms: u64 = latency.as_millis().try_into().unwrap_or(u64::MAX);
                    span.record("failure_class", tracing::field::debug(&error));
                    span.record("failure_latency_ms", latency_ms);
                    tracing::warn!(?error, latency_ms, "request failed");
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
