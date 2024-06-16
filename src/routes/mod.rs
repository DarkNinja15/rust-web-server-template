use std::sync::Arc;

use axum::Router;

use crate::db::DbPool;

pub mod health;
pub mod user;

pub fn create_routes(pool: Arc<DbPool>)->Router{
    Router::new().merge(health::routes()).merge(user::routes(pool))
}