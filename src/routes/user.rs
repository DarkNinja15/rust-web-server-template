use std::sync::Arc;

use axum::{routing::post, Extension, Router};

use crate::{db::DbPool, handlers::user::create_user};

pub fn routes(pool: Arc<DbPool>)->Router{
    Router::new().route("/users", post(create_user).layer(Extension(pool)))
}