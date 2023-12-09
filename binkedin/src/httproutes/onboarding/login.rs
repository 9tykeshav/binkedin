use axum::{
    debug_handler,
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::query_as;

pub fn router<S>(state: crate::Ctx) -> Router<S> {
    let router = Router::new()
        .route("/login", get(login_handler))
        .route("/register", post(register))
        .with_state(state);
    router
}

async fn login_handler(
    State(ctx): State<crate::Ctx>,
    Json(payload): Json<LoginData>,
) -> (StatusCode, Json<User>) {
    let data = query_as!(
        DbData,
        "SELECT * FROM users WHERE email = $1",
        payload.email
    )
    .fetch_one(&ctx.db)
    .await
    .unwrap_or(DbData {
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
    Json(payload): Json<LoginData>,
) -> (StatusCode, Json<RegisterUserData>) {
    let query = sqlx::query!(
        "INSERT INTO users VALUES($1, $2)",
        payload.email,
        payload.password
    )
    .execute(&ctx.db)
    .await;
    match query {
        Ok(_pg_query_result) => (
            StatusCode::CREATED,
            Json(RegisterUserData {
                email: payload.email,
                password: payload.password,
                info: "SUCCESS".to_string(),
            }),
        ),
        Err(error) => {
            let errr = error.as_database_error();
            match errr.unwrap().kind() {
                sqlx::error::ErrorKind::UniqueViolation => (
                    StatusCode::CONFLICT,
                    Json(RegisterUserData {
                        email: payload.email,
                        password: payload.password,
                        info: "USER ALREADY EXISTS".to_string(),
                    }),
                ),
                _ => {
                    println!("{}", errr.unwrap());
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(RegisterUserData {
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
struct LoginData {
    email: String,
    password: String,
}

#[derive(Debug)]
struct DbData {
    email: Option<String>,
    password: Option<String>,
}

#[derive(Deserialize, Serialize)]
struct RegisterUserData {
    email: String,
    password: String,
    info: String,
}
