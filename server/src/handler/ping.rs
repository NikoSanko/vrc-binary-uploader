use axum_extra::extract::{CookieJar, Host};
use generated::apis;
use generated::models;
use http::Method;
use log::info;

/// 疎通確認
pub async fn handle(
    _method: &Method,
    _host: &Host,
    _cookies: &CookieJar,
) -> Result<apis::default::PingResponse, ()> {
    info!("ping() called");
    Ok(apis::default::PingResponse::Status200_SuccessfulOperation(
        models::SuccessResponse {
            message: "pong".to_string(),
            data: None,
        },
    ))
}
