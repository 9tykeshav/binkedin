use axum::{
    debug_handler,
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Error, Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{error, Row};
use sqlx::{
    pool,
    postgres::{PgPool, PgPoolOptions},
    query,
};
use sqlx::{postgres::PgQueryResult, query_as};
use std::time::Duration;

use crate::Ctx;

pub fn router<S>(state: crate::Ctx) -> Router<S> {
    let router = Router::new()
        .route("/login", get(login_handler))
        .route("/register", post(register))
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
        does_exits: if data.email == None { false } else { true },
        email: data.email.unwrap_or("NULL".to_string()),
        password: data.password.unwrap_or("NULL".to_string()),
    };
    //TODO: match and return 404 for not existing users
    let status = if user.does_exits {
        StatusCode::OK
    } else {
        StatusCode::NOT_FOUND
    };
    (status, Json(user))
}

#[debug_handler]
async fn register(
    State(ctx): State<crate::Ctx>,
    Json(payload): Json<Login_data>,
) -> (StatusCode, Json<Register_user_data>) {
    let query = sqlx::query!(
        "INSERT INTO users VALUES($1, $2)",
        payload.email,
        payload.password
    )
    .execute(&ctx.db)
    .await;
    match query {
        Ok(PgQueryResult) => (
            StatusCode::CREATED,
            Json(Register_user_data {
                email: payload.email,
                password: payload.password,
                info: "SUCCESS".to_string(),
            }),
        ),
        Err(Error) => {
            let errr = Error.as_database_error();
            match errr.unwrap().kind() {
                sqlx::error::ErrorKind::UniqueViolation => (
                    StatusCode::CONFLICT,
                    Json(Register_user_data {
                        email: payload.email,
                        password: payload.password,
                        info: "USER ALREADY EXISTS".to_string(),
                    }),
                ),
                _ => {
                    println!("{}", errr.unwrap());
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(Register_user_data {
                            email: payload.email,
                            password: payload.password,
                            info: "INTERNAL SERVER ERROR ".to_string(),
                        }),
                    )
                }
            }
        }
    }
}

// hello
#[derive(Serialize)]
struct User {
    email: String,
    password: String,
    does_exits: bool,
}
#[derive(Deserialize, Serialize)]
struct Login_data {
    email: String,
    password: String,
}

#[derive(Debug)]
struct db_data {
    email: Option<String>,
    password: Option<String>,
}

#[derive(Deserialize, Serialize)]
struct Register_user_data {
    email: String,
    password: String,
    info: String,
}
