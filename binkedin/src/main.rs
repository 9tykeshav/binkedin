use axum::Router;

use sqlx::{postgres::PgPoolOptions, Postgres};
mod httproutes;
use dotenv::dotenv;
use std::time::Duration;
use tower_http::cors::{Any, CorsLayer};


#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();
    dotenv().ok();
    // TODO : check all neccesary vars

    let pool: sqlx::Pool<Postgres> = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&std::env::var("DATABASE_URL").unwrap())
        .await
        .expect("can't connect to database");

    let ctx = Ctx { db: pool };
    let cors = CorsLayer::new().allow_methods(Any).allow_origin(Any).allow_headers(Any);

    // build our application with a route
    let app = Router::new()
        .nest("/api", httproutes::posts::post_routes(ctx.clone()))
        .nest("/api", httproutes::comments::comments_routes(ctx.clone()))
        .nest("/api", httproutes::likes::likes_routes(ctx.clone()))
        .layer(axum::middleware::from_fn_with_state(
            ctx.clone(),
            httproutes::authorisation_middleware::authorisation_middleware_function,
        ))
        .nest("/onboarding", httproutes::onboarding::login::router(ctx)).layer(cors);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Clone)]
struct Ctx {
    db: sqlx::Pool<Postgres>,
}
