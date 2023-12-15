use axum::Router;
use serde::Serialize;
use sqlx::{postgres::PgPoolOptions, Postgres};
mod httproutes;

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
        .nest("/api", httproutes::posts::post_routes(ctx.clone()))
        .nest("/api", httproutes::comments::comments_routes(ctx.clone()))
        .nest("/api", httproutes::likes::likes_routes(ctx.clone()))
        .layer(axum::middleware::from_fn_with_state(
            ctx.clone(),
            httproutes::authorisation_middleware::authorisation_middleware_function,
        ))
        .nest("/onboarding", httproutes::onboarding::login::router(ctx));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Serialize)]
struct User {
    username: String,
    password: String,
}

#[derive(Clone)]
struct Ctx {
    db: sqlx::Pool<Postgres>,
}
