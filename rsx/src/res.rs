use crate::WebResult;

/// Returns a 200 OK response with the given data
pub fn ok<T: ToString>(body: T) -> WebResult {
    Ok(
        some(200 as u16)
        .content_type(tide::http::mime::HTML)
        .body(body.to_string())
        .build()
    )
    .into()
}

/// Returns a new [`ResponseBuilder`](`tide::ResponseBuilder`) with the given status code
pub fn some<Num: Into<u16>>(status: Num) -> tide::ResponseBuilder {
    tide::Response::builder(status.into())
        .header("Server", crate::consts::SERVER)
}

/// Returns specifyable response with the given data
#[allow(non_snake_case)]
pub fn err<T: ToString, Num: Into<u16>>(body: T, status: Num) -> WebResult {
    let status: u16 = status.into();
    if status == 200 {
        ok(body)
    } else {
        Ok(
            some(status)
            .content_type(tide::http::mime::HTML)
            .body(body.to_string())
            .build()
        )
        .into()
    }
}
