use axum::extract::State;
use common_base::http_response::Response;
use serde::Serialize;

use crate::server::http::server::HttpServerState;

pub async fn index(State(_): State<HttpServerState>) -> String {
    success_response("{}")
}

pub fn success_response<T: Serialize>(data: T) -> String {
    let resp = Response { code: 0, data };
    serde_json::to_string(&resp).unwrap()
}
