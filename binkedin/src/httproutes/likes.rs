use axum::{
    self,
    body::Body,
    debug_handler,
    extract::State,
    http::{HeaderMap, Response, StatusCode},
    response::IntoResponse,
    routing::post,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{query, query_as};
use tracing::{event, Level};

use crate::Ctx;

pub fn likes_routes<S>(state: crate::Ctx) -> Router<S> {
    Router::new()
        .route(
            "/like",
            post(handle_post_likes).get(get_likes).delete(delete_like),
        )
        .with_state(state)
}

async fn handle_post_likes(
    State(ctx): State<Ctx>,
    headers: HeaderMap,
    Json(payload): Json<Requestlike>,
) -> Result<StatusCode, ApiError> {
    let author_email = headers.get("email").unwrap().to_str().unwrap().to_string();
    sqlx::query!(
        "INSERT INTO likes (author_email, post_id) VALUES ($1,$2)",
        author_email,
        payload.post_id
    )
    .execute(&ctx.db)
    .await?;

    sqlx::query!(
        "UPDATE posts SET post_like_count = post_like_count + 1 WHERE post_id = $1",
        payload.post_id
    )
    .execute(&ctx.db)
    .await?;

    Ok(StatusCode::CREATED)
}
#[debug_handler]
async fn get_likes(
    State(ctx): State<Ctx>,
    Json(payload): Json<Requestlike>,
) -> Result<Json<Vec<Likes>>, ApiError> {
    let likes = Json(
        query_as!(
            Likes,
            "SELECT author_email from likeS WHERE post_id = $1",
            payload.post_id
        )
        .fetch_all(&ctx.db)
        .await?,
    );

    Ok(likes)
}
#[debug_handler]
async fn delete_like(
    headers: HeaderMap,
    State(ctx): State<Ctx>,
    Json(payload): Json<Requestlike>,
) -> Result<StatusCode, ApiError> {
    let author_email = headers.get("email").unwrap().to_str().unwrap().to_string();
    query!(
        "DELETE FROM likes WHERE author_email = $1 AND post_id = $2",
        author_email,
        payload.post_id,
    )
    .execute(&ctx.db)
    .await?;

    sqlx::query!(
        "UPDATE posts SET post_like_count = post_like_count - 1 WHERE post_id = $1",
        payload.post_id
    )
    .execute(&ctx.db)
    .await?;

    Ok(StatusCode::OK)
}

#[derive(Deserialize, Serialize)]
struct Requestlike {
    post_id: i32,
}

#[derive(Deserialize, Serialize)]
struct Likes {
    author_email: String,
}

enum ApiError {
    DbError(sqlx::error::Error),
}

impl From<sqlx::Error> for ApiError {
    fn from(value: sqlx::error::Error) -> Self {
        ApiError::DbError(value)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (status_code, error_message) = match self {
            Self::DbError(e) => match e {
                sqlx::Error::RowNotFound => (StatusCode::NOT_FOUND, "ENTRY NOT FOUND".to_string()),
                sqlx::Error::Database(e) => match e.kind() {
                    sqlx::error::ErrorKind::ForeignKeyViolation => (
                        StatusCode::NOT_FOUND,
                        "most likely invalild post id".to_string(),
                    ),

                    sqlx::error::ErrorKind::UniqueViolation => (
                        StatusCode::BAD_REQUEST,
                        "can not like a post twice.".to_string(),
                    ),

                    _ => {
                        event!(Level::ERROR, error = e.to_string());
                        (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            "unimplemented DataBaseError".to_string(),
                        )
                    }
                },
                _ => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            },
            //might need in future.
            // _ => (
            //     StatusCode::INTERNAL_SERVER_ERROR,
            //     "Error not implemented".to_string(),
            // ),
        };

        let epic = json!({
            "info" : error_message
        });
        let response = Response::new(Body::new(epic.to_string().to_string()));
        let (mut parts, body) = response.into_parts();
        parts.status = status_code;
        let response = Response::from_parts(parts, body);
        response
    }
}
