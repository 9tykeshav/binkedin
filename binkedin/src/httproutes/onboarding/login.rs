use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::query_as;
use sqlx::{
    pool,
    postgres::{PgPool, PgPoolOptions},
    query,
};

use sqlx::Row;
use std::time::Duration;

use crate::Ctx;

pub fn router<S>(state: crate::Ctx) -> Router<S> {
    let router = Router::new()
        .route("/login", get(login_handler))
        .with_state(state);
    router
}

async fn login_handler(
    State(ctx): State<crate::Ctx>,
    Json(payload): Json<Login_data>,
) -> (StatusCode, Json<User>) {
    let mut does_exist = false;
    let data = query_as!(
        db_data,
        "SELECT * FROM users WHERE email = $1",
        payload.email
    )
    .fetch_one(&ctx.db)
    .await
    .unwrap_or(db_data {
        email: None,
        password: None,
    });

    let user = User {
        email: data.email.clone().unwrap_or("NULL".to_string()),
        password: data.password.unwrap_or("NULL".to_string()),
        does_exits: if data.email == None { false } else { true },
    };
    (StatusCode::OK, Json(user))
}
// hello
#[derive(Serialize)]
struct User {
    email: String,
    password: String,
    does_exits: bool,
}
#[derive(Deserialize)]
struct Login_data {
    email: String,
    password: String,
}

#[derive(Debug)]
struct db_data {
    email: Option<String>,
    password: Option<String>,
}
