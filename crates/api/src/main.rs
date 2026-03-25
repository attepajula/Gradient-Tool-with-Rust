use std::net::SocketAddr;

use axum::{extract::DefaultBodyLimit, routing::post, Router};
use tower_http::{cors::CorsLayer, services::{ServeDir, ServeFile}};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod error;
mod models;
mod routes;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "api=debug,info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let static_dir = std::env::var("STATIC_DIR").unwrap_or_else(|_| "frontend/dist".into());

    let serve_frontend = ServeDir::new(&static_dir)
        .not_found_service(ServeFile::new(format!("{static_dir}/index.html")));

    let allowed_origin = std::env::var("ALLOWED_ORIGIN")
        .unwrap_or_else(|_| "http://localhost:5173".into());

    let cors = CorsLayer::new()
        .allow_origin(allowed_origin.parse::<axum::http::HeaderValue>().unwrap())
        .allow_methods([axum::http::Method::POST, axum::http::Method::GET])
        .allow_headers([axum::http::header::CONTENT_TYPE]);

    let app = Router::new()
        .route("/api/gradient/from-colors", post(routes::gradient::from_colors))
        .route("/api/gradient/render", post(routes::render::render))
        .route("/api/image/extract-colors", post(routes::image::extract_colors))
        .fallback_service(serve_frontend)
        .layer(DefaultBodyLimit::max(11 * 1024 * 1024)) // 11 MB hard cap at HTTP level
        .layer(cors);

    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3000);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("listening on http://{addr}");
    tracing::info!("serving frontend from {static_dir}");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
