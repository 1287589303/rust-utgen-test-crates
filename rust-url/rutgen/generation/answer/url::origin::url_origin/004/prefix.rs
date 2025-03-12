// Answer 0

#[test]
fn test_url_origin_http() {
    let url = Url::parse("http://example.com:80").unwrap();
    let _ = url_origin(&url);
}

#[test]
fn test_url_origin_https() {
    let url = Url::parse("https://example.com:443").unwrap();
    let _ = url_origin(&url);
}

#[test]
fn test_url_origin_ws() {
    let url = Url::parse("ws://example.com:3000").unwrap();
    let _ = url_origin(&url);
}

#[test]
fn test_url_origin_wss() {
    let url = Url::parse("wss://example.com:3001").unwrap();
    let _ = url_origin(&url);
}

#[test]
fn test_url_origin_http_localhost() {
    let url = Url::parse("http://localhost:8080").unwrap();
    let _ = url_origin(&url);
}

#[test]
fn test_url_origin_https_localhost() {
    let url = Url::parse("https://localhost:8443").unwrap();
    let _ = url_origin(&url);
}

