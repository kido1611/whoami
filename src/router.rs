use std::collections::HashMap;

use axum::http::header;
use axum::{Router, routing::get};

use axum::{Json, extract::Request, response::IntoResponse};
use axum_client_ip::ClientIp;
use indexmap::IndexMap;

use crate::utils::is_request_json;

use crate::config::AppConfig;

pub fn setup_router(config: AppConfig) -> Router {
    Router::new()
        .route("/", get(get_home))
        .route("/ip", get(get_ip))
        .route("/user-agent", get(get_user_agent))
        .layer(config.ip_source.into_extension())
}

async fn get_home(ClientIp(ip): ClientIp, req: Request) -> impl IntoResponse {
    let mut headers: IndexMap<String, String> = req
        .headers()
        .iter()
        .filter(|(_, value)| value.to_str().is_ok())
        .map(|(name, value)| (name.to_string(), value.to_str().unwrap().to_string()))
        .collect::<IndexMap<String, String>>();

    headers.shift_insert(0, "ip".to_string(), ip.to_string());

    if is_request_json(req.headers()) {
        return Json(headers).into_response();
    }

    // TODO: HTML view

    headers
        .iter()
        .map(|(key, value)| format!("{}: {}", key, value))
        .collect::<Vec<String>>()
        .join("\n")
        .into_response()
}

async fn get_ip(ClientIp(ip): ClientIp, req: Request) -> impl IntoResponse {
    if is_request_json(req.headers()) {
        let mut map: HashMap<&str, String> = HashMap::new();
        map.insert("ip", ip.to_string());

        return Json(map).into_response();
    }

    ip.to_string().into_response()
}

async fn get_user_agent(req: Request) -> impl IntoResponse {
    let user_agent = match req.headers().get(header::USER_AGENT) {
        Some(header_value) => header_value.to_str().unwrap_or("").to_string(),
        None => "".to_string(),
    };

    if is_request_json(req.headers()) {
        let mut map: HashMap<&str, String> = HashMap::new();
        map.insert("user-agent", user_agent);

        return Json(map).into_response();
    }

    user_agent.into_response()
}
