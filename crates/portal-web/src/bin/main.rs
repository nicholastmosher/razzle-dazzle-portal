use axum::{
    routing::{get, post},
    Router,
};
use dropper_web::upload;
use tracing_subscriber::prelude::*;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let _ = dotenv::dotenv();

    if let None | Some("") = std::env::var("RUST_LOG").ok().as_deref() {
        std::env::set_var("RUST_LOG", "info,dropper_web=trace");
    }

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer())
        .try_init()?;

    tracing::info!("Started portal-web");

    let app = Router::new()
        .route("/", get(|| async { "Hello, world!" }))
        .route("/upload", post(upload));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
