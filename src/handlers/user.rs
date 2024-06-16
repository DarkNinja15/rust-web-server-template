use std::sync::Arc;

use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Extension;
use axum::{extract::Json, response::Json as JsonResponse};
use diesel::RunQueryDsl;
use crate::db::DbPool;

use crate::models::user::User;

pub async fn create_user(Json(user): Json<User>,Extension(pool): Extension<Arc<DbPool>>,) -> impl IntoResponse {
    tracing::debug!("{:#?}",user);
    let mut conn = pool.get().expect("Failed to get database connection from pool");

    match diesel::insert_into(crate::schema::users::table)
        .values(&user)
        .get_result::<User>(&mut conn) {
        Ok(inserted_user) => (StatusCode::CREATED, JsonResponse(inserted_user)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Error inserting user into database").into_response(),
    }
}