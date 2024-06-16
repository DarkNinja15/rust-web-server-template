use std::net::SocketAddr;
use tracing_subscriber::{self, EnvFilter};
use web_server::routes::create_routes;
use web_server::config;

#[tokio::main]
async fn main() {
    config::init();
    tracing_subscriber::fmt().with_env_filter(EnvFilter::from("debug")).init();

    let app=create_routes();

    let addr=SocketAddr::from(([127,0,0,1],3000));
    tracing::debug!("listening on {}",addr);
    

    axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
}
