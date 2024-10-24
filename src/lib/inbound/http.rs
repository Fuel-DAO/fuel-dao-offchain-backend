/*!
    Module `http` exposes an HTTP server that handles HTTP requests to the application. Its
    implementation is opaque to module consumers.
*/

use std::{net::SocketAddr, sync::Arc};

use anyhow::Context;
use axum::{routing::{get, post}, Router};
use reqwest::StatusCode;
use tokio::net;

use crate::domain::transactions::ports::TransactionService; // Update this to your correct path
use super::handlers::create_transaction::create_transaction; // Update this to your correct path



/// Configuration for the HTTP server.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HttpServerConfig<'a> {
    pub port: &'a str,
}

#[derive(Debug, Clone)]
/// The global application state shared between all request handlers.
pub struct AppState<TS: TransactionService> {
    pub transaction_service: Arc<TS>,
}

/// The application's HTTP server. The underlying HTTP package is opaque to module consumers.
pub struct HttpServer {
    router: axum::Router,
    listener: net::TcpListener,
}

impl HttpServer {
    /// Returns a new HTTP server bound to the port specified in `config`.
    pub async fn new(
        transaction_service: impl TransactionService,
        config: HttpServerConfig<'_>,
    ) -> anyhow::Result<Self> {
        let trace_layer = tower_http::trace::TraceLayer::new_for_http().make_span_with(
            |request: &axum::extract::Request<_>| {
                let uri = request.uri().to_string();
                tracing::info_span!("http_request", method = ?request.method(), uri)
            },
        );

        // Construct dependencies to inject into handlers.
        let state = AppState {
            transaction_service: Arc::new(transaction_service),
        };

        let router = axum::Router::new()
            .route("/health", get(health_route))
            .nest("/api", api_routes())
            .layer(trace_layer)
            .with_state(state);

        let addr = SocketAddr::from(([0, 0, 0, 0, 0, 0, 0, 0], config.port.parse::<u16>()?));


        let listener = net::TcpListener::bind(&addr)
            .await
            .with_context(|| format!("failed to listen on port {}", config.port))?;

        Ok(Self { router, listener })
    }

    /// Runs the HTTP server.
    pub async fn run(self) -> anyhow::Result<()> {
        tracing::debug!("listening on {}", self.listener.local_addr().unwrap());
        axum::serve(self.listener, self.router)
            .await
            .context("received error from running server")?;
        Ok(())
    }
}

fn api_routes<TS: TransactionService>() -> Router<AppState<TS>> {
    Router::new().route("/transactions", post(create_transaction::<TS>)) // Route for creating transactions
}

async fn health_route() -> (StatusCode, &'static str) {
    (StatusCode::OK, "OK")
}