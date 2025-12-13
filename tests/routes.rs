use std::net::SocketAddr;

use axum::{
    body::Body,
    extract::ConnectInfo,
    http::{Request, StatusCode, header},
};
use http_body_util::BodyExt;
use tower::ServiceExt;
use whoami::{config::AppConfig, router::setup_router};

#[tokio::test]
async fn should_be_ok_when_accessed_with_default_config() {
    let app_config = AppConfig::new().unwrap();
    let router = setup_router(app_config);

    let mock_addr: SocketAddr = "1.2.3.4:12345".parse().unwrap();
    let connect_info = ConnectInfo(mock_addr);
    let mut req = Request::builder().uri("/").body(Body::empty()).unwrap();
    req.extensions_mut().insert(connect_info);

    let response = router.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let content_type = response
        .headers()
        .get(header::CONTENT_TYPE)
        .unwrap()
        .to_str()
        .unwrap();
    assert!(content_type.contains("text/plain"));

    let body = response.into_body().collect().await.unwrap().to_bytes();
    assert_eq!(body, "ip: 1.2.3.4");
}

#[tokio::test]
async fn home_should_be_return_correctly_as_html() {
    let app_config = AppConfig::new().unwrap();
    let router = setup_router(app_config);

    let mock_addr: SocketAddr = "1.2.3.4:12345".parse().unwrap();
    let connect_info = ConnectInfo(mock_addr);
    let mut req = Request::builder()
        .uri("/")
        .header(header::ACCEPT, "text/html")
        .body(Body::empty())
        .unwrap();
    req.extensions_mut().insert(connect_info);

    let response = router.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let content_type = response
        .headers()
        .get(header::CONTENT_TYPE)
        .unwrap()
        .to_str()
        .unwrap();
    assert!(content_type.contains("text/html"));

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body_string = String::from_utf8(body.to_vec()).unwrap();
    assert!(body_string.contains("1.2.3.4"));
}

#[tokio::test]
async fn home_should_be_return_correctly_as_json() {
    let app_config = AppConfig::new().unwrap();
    let router = setup_router(app_config);

    let mock_addr: SocketAddr = "127.0.0.1:12345".parse().unwrap();
    let connect_info = ConnectInfo(mock_addr);
    let mut req = Request::builder()
        .uri("/")
        .header(header::ACCEPT, "application/json")
        .body(Body::empty())
        .unwrap();
    req.extensions_mut().insert(connect_info);

    let response = router.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let content_type = response
        .headers()
        .get(header::CONTENT_TYPE)
        .unwrap()
        .to_str()
        .unwrap();
    assert!(content_type.contains("application/json"));

    let body = response.into_body().collect().await.unwrap().to_bytes();
    assert_eq!(body, r#"{"ip":"127.0.0.1","accept":"application/json"}"#);
}

#[tokio::test]
async fn should_be_ok_when_using_custom_ip_source() {
    let app_config = AppConfig {
        ip_source: axum_client_ip::ClientIpSource::CfConnectingIp,
        port: 8080,
    };
    let router = setup_router(app_config);

    let mock_addr: SocketAddr = "127.0.0.1:12345".parse().unwrap();
    let connect_info = ConnectInfo(mock_addr);
    let mut req = Request::builder()
        .uri("/")
        .header("Cf-Connecting-Ip", "192.168.1.200")
        .body(Body::empty())
        .unwrap();
    req.extensions_mut().insert(connect_info);

    let response = router.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let content_type = response
        .headers()
        .get(header::CONTENT_TYPE)
        .unwrap()
        .to_str()
        .unwrap();
    assert!(content_type.contains("text/plain"));

    let body = response.into_body().collect().await.unwrap().to_bytes();
    assert_eq!(body, "ip: 192.168.1.200\ncf-connecting-ip: 192.168.1.200");
}

#[tokio::test]
async fn should_be_error_when_header_is_missing_and_using_custom_ip_source() {
    let app_config = AppConfig {
        ip_source: axum_client_ip::ClientIpSource::CfConnectingIp,
        port: 8080,
    };
    let router = setup_router(app_config);

    let mock_addr: SocketAddr = "127.0.0.1:12345".parse().unwrap();
    let connect_info = ConnectInfo(mock_addr);
    let mut req = Request::builder().uri("/").body(Body::empty()).unwrap();
    req.extensions_mut().insert(connect_info);

    let response = router.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn ip_should_be_return_correctly_as_text() {
    let app_config = AppConfig::new().unwrap();
    let router = setup_router(app_config);

    let mock_addr: SocketAddr = "127.0.0.1:12345".parse().unwrap();
    let connect_info = ConnectInfo(mock_addr);
    let mut req = Request::builder().uri("/ip").body(Body::empty()).unwrap();
    req.extensions_mut().insert(connect_info);

    let response = router.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let content_type = response
        .headers()
        .get(header::CONTENT_TYPE)
        .unwrap()
        .to_str()
        .unwrap();
    assert!(content_type.contains("text/plain"));

    let body = response.into_body().collect().await.unwrap().to_bytes();
    assert_eq!(body, r#"127.0.0.1"#);
}

#[tokio::test]
async fn ip_should_be_return_correctly_as_json() {
    let app_config = AppConfig::new().unwrap();
    let router = setup_router(app_config);

    let mock_addr: SocketAddr = "127.0.0.1:12345".parse().unwrap();
    let connect_info = ConnectInfo(mock_addr);
    let mut req = Request::builder()
        .uri("/ip")
        .header(header::ACCEPT, "application/json")
        .body(Body::empty())
        .unwrap();
    req.extensions_mut().insert(connect_info);

    let response = router.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let content_type = response
        .headers()
        .get(header::CONTENT_TYPE)
        .unwrap()
        .to_str()
        .unwrap();
    assert!(content_type.contains("application/json"));

    let body = response.into_body().collect().await.unwrap().to_bytes();
    assert_eq!(body, r#"{"ip":"127.0.0.1"}"#);
}

#[tokio::test]
async fn user_agent_should_be_return_correctly_as_text() {
    let app_config = AppConfig::new().unwrap();
    let router = setup_router(app_config);

    let mock_addr: SocketAddr = "127.0.0.1:12345".parse().unwrap();
    let connect_info = ConnectInfo(mock_addr);
    let mut req = Request::builder()
        .uri("/user-agent")
        .header(header::USER_AGENT, "Curl")
        .body(Body::empty())
        .unwrap();
    req.extensions_mut().insert(connect_info);

    let response = router.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let content_type = response
        .headers()
        .get(header::CONTENT_TYPE)
        .unwrap()
        .to_str()
        .unwrap();
    assert!(content_type.contains("text/plain"));

    let body = response.into_body().collect().await.unwrap().to_bytes();
    assert_eq!(body, r#"Curl"#);
}

#[tokio::test]
async fn user_agent_should_be_return_correctly_as_json() {
    let app_config = AppConfig::new().unwrap();
    let router = setup_router(app_config);

    let mock_addr: SocketAddr = "127.0.0.1:12345".parse().unwrap();
    let connect_info = ConnectInfo(mock_addr);
    let mut req = Request::builder()
        .uri("/user-agent")
        .header(header::USER_AGENT, "Curl")
        .header(header::ACCEPT, "application/json")
        .body(Body::empty())
        .unwrap();
    req.extensions_mut().insert(connect_info);

    let response = router.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let content_type = response
        .headers()
        .get(header::CONTENT_TYPE)
        .unwrap()
        .to_str()
        .unwrap();
    assert!(content_type.contains("application/json"));

    let body = response.into_body().collect().await.unwrap().to_bytes();
    assert_eq!(body, r#"{"user-agent":"Curl"}"#);
}

#[tokio::test]
async fn user_agent_should_be_ok_when_user_agent_is_empty() {
    let app_config = AppConfig::new().unwrap();
    let router = setup_router(app_config);

    let mock_addr: SocketAddr = "127.0.0.1:12345".parse().unwrap();
    let connect_info = ConnectInfo(mock_addr);
    let mut req = Request::builder()
        .uri("/user-agent")
        .body(Body::empty())
        .unwrap();
    req.extensions_mut().insert(connect_info);

    let response = router.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let content_type = response
        .headers()
        .get(header::CONTENT_TYPE)
        .unwrap()
        .to_str()
        .unwrap();
    assert!(content_type.contains("text/plain"));

    let body = response.into_body().collect().await.unwrap().to_bytes();
    assert_eq!(body, r#""#);
}
