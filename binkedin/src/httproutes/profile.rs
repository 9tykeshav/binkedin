use axum::routing::get;
use axum::{
    debug_handler,
    extract::{Query, State},
    http::StatusCode,
    Json, Router,
};
use core::panic;

use serde::{Deserialize, Serialize};
use sqlx::query_as;

pub fn home_routes<S>(state: crate::Ctx) -> Router<S> {
    Router::new()
        .route("/homefeed", get(get_all_posts))
        .with_state(state)
}

#[debug_handler]
// TODO IMPLEMENT NOT FOUND FOR NOT FOUND USERS, this belongs to profile routes
async fn get_all_posts(
    State(ctx): State<crate::Ctx>,
    payload: Query<GetPostsData>,
) -> Result<(StatusCode, Json<Vec<Post>>), (StatusCode, Json<String>)> {
    let email = payload.email.clone();
    let data_fetched = query_as!(
        Post,
        "SELECT user_email, post_id ,caption ,image_url 
                                ,post_like_count ,post_comment_count 
                                ,post_time FROM posts WHERE user_email = $1 ORDER BY post_id DESC",
        email
    )
    .fetch_all(&ctx.db)
    .await;

    match data_fetched {
        Ok(d) => {
            return Ok((StatusCode::FOUND, Json(d)));
        }
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string()))),
    }
}

#[derive(Serialize, Deserialize)]
struct GetPostsData {
    email: String,
}
#[derive(Serialize, Deserialize)]
struct Post {
    user_email: String,
    post_id: i32,
    caption: Option<String>,
    image_url: Option<String>,
    post_like_count: i32,
    post_comment_count: i32,
    post_time: String,
}
