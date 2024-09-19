use axum::{
    body::Bytes,
    debug_handler,
    extract::{Multipart, Query, State},
    http::{HeaderMap, StatusCode},
    routing::post,
    Json, Router,
};
use core::panic;
use image;
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as};
use std::path;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid;

pub fn post_routes<S>(state: crate::Ctx) -> Router<S> {
    Router::new()
        .route("/post", post(handle_post).get(get_post).delete(delete_post))
        .with_state(state)
}

#[debug_handler]
async fn handle_post(
    State(ctx): State<crate::Ctx>,
    headers: HeaderMap,
    mut multipart: Multipart,
) -> Result<StatusCode, (StatusCode, Json<ErrorInfo>)> {
    let mut image_bytes = Bytes::default();
    let mut json: Json<JsonData> = Json(JsonData {
        caption: "".to_string(),
    });
    while let Some(field) = multipart.next_field().await.unwrap() {
        println!("we are at the top of the while loop!!");
        let name = field.name().unwrap().to_string();
        if name == "caption" {
            json = Json(JsonData {
                caption: std::str::from_utf8(&field.bytes().await.unwrap())
                    .unwrap()
                    .to_string(),
            });
            println!("{:?}", json)
        } else if name == "image" {
            image_bytes = field.bytes().await.unwrap();
        } else {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(ErrorInfo {
                    info: "please enter some data??".to_string(),
                }),
            ));
        }
    }

    let dyn_image;
    let post_path = uuid::Uuid::new_v4().to_string();
    let email = headers.get("email").unwrap().to_str().unwrap().to_string();
    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
        .to_string();

    if !image_bytes.is_empty() {
        match image::load_from_memory_with_format(&image_bytes, image::ImageFormat::Png) {
            Ok(image) => {
                dyn_image = image;

                match dyn_image.save_with_format(
                    path::Path::new(&format!(
                        "{}\\{}.png",
                        (&std::env::var("BINKEDIN_MEDIA").unwrap()),
                        post_path
                    )),
                    image::ImageFormat::Png,
                ) {
                    Ok(_) => {
                        let query_result = query!(
                            "
                            INSERT INTO posts
                            (user_email, caption, image_url, 
                            post_like_count, post_comment_count, 
                            post_time) 
                            VALUES ($1, $2, $3,$4,$5,$6)",
                            email,
                            json.caption,
                            post_path,
                            0,
                            0,
                            time
                        )
                        .execute(&ctx.db)
                        .await;

                        match query_result {
                            Ok(o) => {
                                println!("{:?}", o);
                                Ok(StatusCode::CREATED)
                            }
                            Err(error) => {
                                return Err((
                                    StatusCode::INTERNAL_SERVER_ERROR,
                                    Json(ErrorInfo {
                                        info: error.to_string(),
                                    }),
                                ))
                            }
                        }
                    }
                    Err(e) => {
                        return Err((
                            StatusCode::BAD_REQUEST,
                            Json(ErrorInfo {
                                info: e.to_string(),
                            }),
                        ));
                    }
                }
            }
            Err(e) => {
                return Err((
                    StatusCode::BAD_REQUEST,
                    Json(ErrorInfo {
                        info: e.to_string(),
                    }),
                ));
            }
        }
    } else {
        let query_result = query!(
            "
                            INSERT INTO posts
                            (user_email, caption, 
                            post_like_count, post_comment_count, 
                            post_time) 
                            VALUES ($1, $2, $3,$4,$5)",
            email,
            json.caption,
            0,
            0,
            time
        )
        .execute(&ctx.db)
        .await;

        match query_result {
            Ok(o) => {
                println!("{:?}", o);
                return Ok(StatusCode::CREATED);
            }
            Err(error) => {
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorInfo {
                        info: error.to_string(),
                    }),
                ))
            }
        }
    }
}

#[debug_handler]
async fn get_post(
    State(ctx): State<crate::Ctx>,
    payload: Query<GetPostData>,
) -> Result<(StatusCode, Json<PostData>), StatusCode> {
    let response = query_as!(
        PostData,
        "SELECT user_email, post_id ,caption ,image_url 
                                ,post_like_count ,post_comment_count 
                                ,post_time FROM posts WHERE post_id = $1 ",
        payload.postid
    )
    .fetch_one(&ctx.db)
    .await;

    match response {
        Ok(data) => return Ok((StatusCode::FOUND, Json(data))),
        Err(e) => {
            println!("{:?}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }
}

async fn delete_post(
    headers: HeaderMap,
    State(ctx): State<crate::Ctx>,
    Json(payload): Json<DelPostData>,
) -> Result<StatusCode, (StatusCode, Json<String>)> {
    let username = headers.get("email").unwrap().to_str().unwrap().to_string();
    let db_response = query!(
        "DELETE FROM POSTS WHERE post_id = $1 AND user_email = $2",
        payload.post_id,
        username
    )
    .execute(&ctx.db)
    .await;

    match db_response {
        Ok(_) => return Ok(StatusCode::OK),
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string()))),
    }
}

// #[debug_handler]
// async fn get_post( headers: HeaderMap,
//     State(ctx): State<crate::Ctx>,
//     post_id : Query<PostId>,
// )-> Result<(StatusCode, Json<PostData>), StatusCode> {

//     let data_fetched = query_as!(PostData,
//         "SELECT user_email, posts.post_id ,caption ,image_url
//                                 ,post_like_count ,post_comment_count
//                                 ,post_time, comments.content AS comment_data FROM posts JOIN comments ON posts.post_id = comments.post_id "
//     )
//     .fetch_one(&ctx.db)
//     .await;

//     println!("{:?}", post_id);
//     Err(StatusCode::OK)

// }

#[derive(Serialize, Deserialize)]
struct PostData {
    user_email: String,
    post_id: i32,
    caption: Option<String>,
    image_url: Option<String>,
    post_like_count: i32,
    post_comment_count: i32,
    post_time: String,
}
#[derive(Serialize, Deserialize)]
struct ErrorInfo {
    info: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct JsonData {
    caption: String,
}
#[derive(Serialize, Deserialize)]
struct GetPostsData {
    email: String,
}

#[derive(Serialize, Deserialize)]
struct GetPostData {
    postid: i32,
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
#[derive(Serialize, Deserialize)]
struct DelPostData {
    post_id: i32,
}
#[derive(Serialize, Deserialize, Debug)]
struct PostId {
    post_id: i32,
}
