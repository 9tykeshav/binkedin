use axum::{
    body::Body,
    extract::{Request, State},
    http::{self, response, HeaderMap, StatusCode},
    middleware::{self, Next},
    response::{ErrorResponse, IntoResponse, Response},
    routing::get,
    Error, Json, Router,
};

use core::panic;
use serde::{Deserialize, Serialize};
use sqlx::{error, Row};
use sqlx::{
    pool,
    postgres::{PgPool, PgPoolOptions},
    query,
};
use sqlx::{postgres::PgQueryResult, query_as};
use std::time::Duration;

pub async fn authorisation_middleware_function(
    State(ctx): State<crate::Ctx>,
    headers: HeaderMap,
    request: Request,
    next: Next,
) -> impl IntoResponse {
    // do something with `request`...

    if headers.contains_key("password") && headers.contains_key("email") {
        let email = headers.get("email").unwrap();
        let password_recived = headers.get("password").unwrap();
        let password_fetched = query_as!(
            Password,
            "SELECT password FROM users WHERE email = $1",
            email.to_str().unwrap()
        )
        .fetch_one(&ctx.db)
        .await;
        match password_fetched {
            Ok(password) => {
                let does_exits = false;
                if password.password == password_recived.to_str().unwrap() {
                    let response = next.run(request).await;

                    // do something with `response`...

                    return response;
                } else {
                    let response = CustomErrorResponse::new(
                        "INVALID EMAIL OR PASSWORD".to_string(),
                        StatusCode::UNAUTHORIZED,
                    );
                    return response.into_response();
                }
            }

            Err(err) => {
                let err = err;
                match err {
                    sqlx::Error::RowNotFound => {
                        let response = CustomErrorResponse::new(
                            "PLEASE REGISTER".to_string(),
                            StatusCode::NOT_FOUND,
                        );
                        return response.into_response();
                    }
                    _ => {
                        println!("NEW ERROR");
                        panic!()
                    }
                }
            }
        }
    } else {
        let response = CustomErrorResponse::new(
            "ADD email And password TO THE HEADERS ".to_string(),
            StatusCode::BAD_REQUEST,
        );
        return response.into_response();
    }
}

struct Password {
    password: String,
}

struct CustomErrorResponse {
    info: String,
    status_code: StatusCode,
}

impl CustomErrorResponse {
    fn new(info: String, status_code: StatusCode) -> Self {
        Self {
            info: info,
            status_code,
        }
    }
}

impl IntoResponse for CustomErrorResponse {
    fn into_response(self) -> Response {
        let response = Response::new(Body::new(self.info));
        let (mut parts, body) = response.into_parts();
        parts.status = self.status_code;
        let response = Response::from_parts(parts, body);
        response
    }
}
