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

pub fn comments_routes<S>(state: crate::Ctx) -> Router<S> {
    Router::new()
        .route(
            "/comment",
            post(handle_post_comment).get(handle_get_comments),
        )
        .with_state(state)
}
#[debug_handler]
async fn handle_post_comment(
    State(ctx): State<crate::Ctx>,
    headers: HeaderMap,
    Json(payload): Json<CommentData>,
) -> Result<StatusCode, ApiError> {
    let author_email = headers.get("email").unwrap().to_str().unwrap();
    query!(
        "INSERT INTO comments (author_email, post_id, content) VALUES ($1,$2,$3);",
        author_email,
        payload.post_id,
        payload.content
    )
    .execute(&ctx.db)
    .await?;
    query!(
        "UPDATE posts SET post_comment_count = post_comment_count +1 WHERE post_id = $1",
        payload.post_id
    )
    .execute(&ctx.db)
    .await?;
    Ok(StatusCode::CREATED)
}

async fn handle_get_comments(
    State(ctx): State<crate::Ctx>,
    Json(payload): Json<GetCommentData>,
) -> Result<Json<Vec<Comment>>, ApiError> {
    let comments = Json(
        query_as!(
            Comment,
            "SELECT * FROM comments WHERE post_id = $1 ",
            payload.post_id
        )
        .fetch_all(&ctx.db)
        .await?,
    );
    Ok(comments)
}

#[derive(Serialize, Deserialize)]
struct CommentData {
    post_id: i32,
    content: String,
}
#[derive(Serialize, Deserialize)]
struct GetCommentData {
    post_id: i32,
}
#[derive(Serialize, Deserialize)]
struct Comment {
    comment_id: i32,
    author_email: String,
    post_id: i32,
    content: String,
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
