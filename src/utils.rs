use std::net::IpAddr;

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

pub fn ip_parser(ip: IpAddr) -> String {
    if ip.is_loopback() {
        "127.0.0.1".to_string()
    } else {
        match ip {
            IpAddr::V4(v4) => v4.to_string(),
            IpAddr::V6(ipv6_addr) => {
                if let Some(v4) = ipv6_addr.to_ipv4() {
                    v4.to_string()
                } else {
                    IpAddr::V6(ipv6_addr).to_string()
                }
            }
        }
    }
}
