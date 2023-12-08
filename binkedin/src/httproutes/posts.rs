use axum::{
    body::{Body, Bytes},
    extract::Multipart,
    extract::{Request, State},
    http::{self, response, HeaderMap, StatusCode},
    middleware::{self, Next},
    response::{ErrorResponse, IntoResponse, Response},
    routing::{get, post, Route},
    Error, Json, Router,
};
use core::panic;
use image;
use serde::{Deserialize, Serialize};
use sqlx::{error, Row};
use sqlx::{
    pool,
    postgres::{PgPool, PgPoolOptions},
    query,
};
use sqlx::{postgres::PgQueryResult, query_as};
use std::{path, string, time::Duration};

use super::onboarding::login::router;

pub fn post_routes<S>(state: crate::Ctx) -> Router<S> {
    Router::new()
        .route("/post", post(handle_post))
        .with_state(state)
}

async fn handle_post(
    mut multipart: Multipart,
) -> Result<StatusCode, (StatusCode, Json<ErrorInfo>)> {
    let mut json_bytes = Bytes::default();
    let mut image_bytes = Bytes::default();
    let mut json: Json<JsonData> = Json(JsonData {
        date: 0,
        caption: "".to_string(),
    });
    while let Some(mut field) = multipart.next_field().await.unwrap() {
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

    match image::load_from_memory_with_format(&image_bytes, image::ImageFormat::Png) {
        Ok(image) => {
            match image.save_with_format(
                path::Path::new("D:\\binkedinMedia\\g.png"),
                image::ImageFormat::Png,
            ) {
                Ok(g) => {}
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

    Ok(StatusCode::OK)
}

#[derive(Serialize, Deserialize)]
struct ErrorInfo {
    info: String,
}
#[derive(Serialize, Deserialize)]
struct JsonData {
    date: i64,
    caption: String,
}
