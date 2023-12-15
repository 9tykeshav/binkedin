use axum::{
    body::Bytes,
    debug_handler,
    extract::Multipart,
    extract::State,
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
        .route("/post", post(handle_post).get(get_posts))
        .with_state(state)
}

#[debug_handler]
async fn handle_post(
    State(ctx): State<crate::Ctx>,
    headers: HeaderMap,
    mut multipart: Multipart,
) -> Result<StatusCode, (StatusCode, Json<ErrorInfo>)> {
    let mut json_bytes = Bytes::default();
    let mut image_bytes = Bytes::default();
    let mut json: Json<JsonData> = Json(JsonData {
        caption: "".to_string(),
    });
    while let Some(field) = multipart.next_field().await.unwrap() {
        println!("we are at the top of the while loop!!");
        let name = field.name().unwrap().to_string();
        if name == "data" {
            json_bytes = field.bytes().await.unwrap();
        } else if name == "image" {
            image_bytes = field.bytes().await.unwrap();
        }
        match Json::<JsonData>::from_bytes(&json_bytes) {
            Ok(j) => {
                json = j;
            }
            Err(er) => {
                return Err((
                    StatusCode::BAD_REQUEST,
                    Json(ErrorInfo {
                        info: er.body_text(),
                    }),
                ));
            }
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
                    path::Path::new(&format!("D:\\binkedinMedia\\{}.png", post_path)),
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
async fn get_posts(
    State(ctx): State<crate::Ctx>,
    Json(payload): Json<GetPostData>,
) -> Result<(StatusCode, Json<Vec<Post>>), (StatusCode, Json<String>)> {
    let email = payload.email;
    let data_fetched = query_as!(
        Post,
        "SELECT post_id ,caption ,image_url 
                                ,post_like_count ,post_comment_count 
                                ,post_time FROM posts WHERE user_email = $1",
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
struct ErrorInfo {
    info: String,
}
#[derive(Serialize, Deserialize)]
struct JsonData {
    caption: String,
}
#[derive(Serialize, Deserialize)]
struct GetPostData {
    email: String,
}
#[derive(Serialize, Deserialize)]
struct Post {
    post_id: i32,
    caption: Option<String>,
    image_url: Option<String>,
    post_like_count: i32,
    post_comment_count: i32,
    post_time: String,
}
