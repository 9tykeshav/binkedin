use axum::{
    extract::State,
    http::StatusCode,
    middleware,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{
    pool,
    postgres::{PgPool, PgPoolOptions},
    query, Postgres,
};
mod httproutes;
use httproutes::onboarding;
use sqlx::Row;
use std::time::Duration;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    let pool: sqlx::Pool<Postgres> = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect("postgres://postgres:1234@localhost/binkedin")
        .await
        .expect("can't connect to database");

    let ctx = Ctx { db: pool };

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        // .route("/", get(root))
        // // `POST /users` goes to `create_user`
        // .route("/users", post(create_user))
        .route("/test", post(create_user))
        .layer(axum::middleware::from_fn_with_state(
            ctx.clone(),
            httproutes::authorisation_middleware::authorisation_middleware_function,
        ))
        .nest("/api", httproutes::onboarding::login::router(ctx));

    // .with_state(ctx);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn root(State(pool): State<PgPool>) -> (StatusCode, Json<User>) {
    let data = sqlx::query!("SELECT * FROM users")
        .fetch_all(&pool)
        .await
        .unwrap();

    println!("{:?}", data);
    let user = User {
        password: "qwe".to_string(),
        username: "gg".to_string(),
    };

    (StatusCode::CREATED, Json(user))
}
async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    // insert your application logic here
    let user = User {
        password: "qwe".to_string(),
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    username: String,
    password: String,
}

#[derive(Clone)]
struct Ctx {
    db: sqlx::Pool<Postgres>,
}
