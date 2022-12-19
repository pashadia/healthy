use axum::{routing::get, Router};
use std::net::SocketAddr;

use clap::Parser;
use tower_http::trace::TraceLayer;
use tracing::Span;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Config {
    #[arg(default_value_t = 8000)]
    port: u16,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(EnvFilter::new(
            std::env::var("LOG_LEVEL").unwrap_or_else(|_| "healthy=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();
    let config = Config::parse();

    let app = Router::new()
        .route("/", get(|| async { "OK" }))
        .layer(TraceLayer::new_for_http());
    let addr = SocketAddr::from(([127, 0, 0, 1], config.port));

    tracing::info!("Listening on port {}", config.port);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
