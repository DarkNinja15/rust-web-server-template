use std::net::SocketAddr;
use std::sync::Arc;
use tracing_subscriber::{self, EnvFilter};
use web_server::{config, db};
use web_server::routes::create_routes;

#[tokio::main]
async fn main() {
    config::init();
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from("debug"))
        .init();

    let pool = Arc::new(db::establish_connection());
    let app = create_routes(pool);


    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
