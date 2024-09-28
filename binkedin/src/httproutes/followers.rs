use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};

use axum::routing::{get, post};
use axum::Json;
use axum::{debug_handler, extract::Path, routing::put, Router};
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as};

pub fn follow_routes<S>(state: crate::Ctx) -> Router<S> {
    Router::new()
        .route("/follow-request", get(handle_get_reqs))
        .route(
            "/follow-request/create/:username",
            post(handle_create_follow),
        )
        .route(
            "/follow-request/accept/:username",
            put(handle_accept_follow),
        )
        .route(
            "/follow-request/reject/:username",
            put(handle_reject_follow),
        )
        .with_state(state)
}

#[debug_handler]
async fn handle_create_follow(
    Path(username): Path<String>,
    headers: HeaderMap,
    State(ctx): State<crate::Ctx>,
) -> (StatusCode, Json<QueryInfo>) {
    let follower = headers.get("email").unwrap().to_str().unwrap();

    let response = query!(
        "INSERT INTO followers (follower_username, followee_username, request_status) VALUES ($1,$2,$3)",
        follower,
        username,
        "pending"
    ).execute(&ctx.db).await;
    match response {
        Ok(v) => (
            StatusCode::CREATED,
            Json(QueryInfo {
                info: v.rows_affected().to_string(),
            }),
        ),
        Err(err) => {
            match err.as_database_error() {
                Some(err) => match err.kind() {
                    sqlx::error::ErrorKind::UniqueViolation => (
                        StatusCode::CONFLICT,
                        Json(QueryInfo {
                            info: "cannot follow a user twice ".to_string(),
                        }),
                    ),
                    //TODO : log these errors somewhere
                    _ => (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(QueryInfo {
                            info: "unimplemented database error".to_string(),
                        }),
                    ),
                },
                None => {
                    todo!()
                }
            }
        }
    }
}

#[debug_handler]
async fn handle_accept_follow(
    Path(username): Path<String>,
    State(ctx): State<crate::Ctx>,
    headers: HeaderMap,
) -> StatusCode {
    let response = query!(
        "UPDATE followers SET request_status = $1 WHERE follower_username = $2 AND followee_username = $3",
        "accept",
        username,
        headers.get("email").unwrap().to_str().unwrap()
    )
    .execute(&ctx.db)
    .await;
    match response {
        Ok(_) => StatusCode::ACCEPTED,
        //TODO: log this error
        Err(_err) => StatusCode::BAD_REQUEST,
    }
}

#[debug_handler]
async fn handle_reject_follow(
    Path(username): Path<String>,
    State(ctx): State<crate::Ctx>,
    headers: HeaderMap,
) -> StatusCode {
    let response = query!(
        "DELETE FROM followers WHERE follower_username = $1 AND followee_username = $2",
        username,
        headers.get("email").unwrap().to_str().unwrap()
    )
    .execute(&ctx.db)
    .await;
    match response {
        Ok(_rows) => StatusCode::ACCEPTED,
        // TODO : log this error
        Err(_err) => StatusCode::BAD_REQUEST,
    }
}

async fn handle_get_reqs(
    headers: HeaderMap,
    State(ctx): State<crate::Ctx>,
) -> Result<(StatusCode, Json<Vec<PendingReqs>>), (StatusCode, Json<QueryInfo>)> {
    let followee = headers.get("email").unwrap().to_str().unwrap();
    let response = query_as!(
        PendingReqs,
        "SELECT follower_username FROM followers WHERE 
         request_status = 'pending' AND followee_username = $1",
        followee
    )
    .fetch_all(&ctx.db)
    .await;

    match response {
        Ok(data) => Ok((StatusCode::FOUND, Json(data))),
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(QueryInfo {
                info: err.to_string(),
            }),
        )),
    }
}

#[derive(Serialize, Deserialize)]
struct PendingReqs {
    follower_username: String,
}

#[derive(Serialize, Deserialize)]
struct QueryInfo {
    info: String,
}
