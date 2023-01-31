use actix_web::{HttpResponse, Result};
use actix_http::StatusCode;

/// Returns a 200 OK response with the given data
#[allow(non_snake_case)]
pub fn Ok<T: ToString>(data: T) -> Result<HttpResponse> {
    actix_web::Result::Ok(
        HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(data.to_string())
    )
}

/// Returns specifyable response with the given data
#[allow(non_snake_case)]
pub fn Err<T: ToString, Num: Into<u16>>(data: T, num: Num) -> Result<HttpResponse> {
    actix_web::Result::Ok(
        HttpResponse::build(StatusCode::from_u16(num.into()).unwrap())
        .content_type("text/html; charset=utf-8")
        .body(data.to_string())
    )
}
