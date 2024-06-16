use axum::Router;

pub mod health;

pub fn create_routes()->Router{
    Router::new().merge(health::routes())
}