use axum::http::{HeaderMap, header};

pub fn is_request_html(headers: &HeaderMap) -> bool {
    check_request_accept(headers, "text/html")
}

pub fn is_request_json(headers: &HeaderMap) -> bool {
    check_request_accept(headers, "application/json")
}

pub fn check_request_accept(headers: &HeaderMap, accept: &str) -> bool {
    match headers.get(header::ACCEPT) {
        Some(value) => match value.to_str() {
            Ok(val) => val
                .to_lowercase()
                .split(",")
                .collect::<Vec<&str>>()
                .contains(&accept),
            Err(_) => false,
        },
        None => false,
    }
}
