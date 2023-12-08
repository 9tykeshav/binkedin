pub mod authorisation_middleware;
pub mod onboarding;
pub mod posts;

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

pub fn post_routes(state: crate::Ctx) -> Router {
    Router::new().merge(posts::post_routes(state.clone()))
}
