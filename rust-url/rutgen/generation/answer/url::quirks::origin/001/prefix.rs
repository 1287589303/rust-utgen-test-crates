// Answer 0

#[test]
fn test_origin_http_with_host() {
    let url = Url {
        serialization: "http://example.com".to_string(),
        scheme_end: 4,
        username_end: 0,
        host_start: 7,
        host_end: 17,
        host: HostInternal::new("example.com".to_string()),
        port: None,
        path_start: 17,
        query_start: None,
        fragment_start: None,
    };
    let _ = origin(&url);
}

#[test]
fn test_origin_https_with_host_and_port() {
    let url = Url {
        serialization: "https://example.com:8080".to_string(),
        scheme_end: 5,
        username_end: 0,
        host_start: 8,
        host_end: 18,
        host: HostInternal::new("example.com".to_string()),
        port: Some(8080),
        path_start: 18,
        query_start: None,
        fragment_start: None,
    };
    let _ = origin(&url);
}

#[test]
fn test_origin_http_with_userinfo() {
    let url = Url {
        serialization: "http://user:pass@example.com".to_string(),
        scheme_end: 4,
        username_end: 10,
        host_start: 15,
        host_end: 25,
        host: HostInternal::new("example.com".to_string()),
        port: None,
        path_start: 25,
        query_start: None,
        fragment_start: None,
    };
    let _ = origin(&url);
}

#[test]
fn test_origin_http_with_empty_host() {
    let url = Url {
        serialization: "http://".to_string(),
        scheme_end: 4,
        username_end: 0,
        host_start: 7,
        host_end: 7,
        host: HostInternal::new("".to_string()),
        port: None,
        path_start: 7,
        query_start: None,
        fragment_start: None,
    };
    let _ = origin(&url);
}

#[test]
fn test_origin_without_port() {
    let url = Url {
        serialization: "http://localhost".to_string(),
        scheme_end: 4,
        username_end: 0,
        host_start: 7,
        host_end: 16,
        host: HostInternal::new("localhost".to_string()),
        port: None,
        path_start: 16,
        query_start: None,
        fragment_start: None,
    };
    let _ = origin(&url);
}

#[test]
fn test_origin_with_special_characters_in_username_and_password() {
    let url = Url {
        serialization: "http://user%20name:pass%40word@example.com".to_string(),
        scheme_end: 4,
        username_end: 16,
        host_start: 27,
        host_end: 37,
        host: HostInternal::new("example.com".to_string()),
        port: None,
        path_start: 37,
        query_start: None,
        fragment_start: None,
    };
    let _ = origin(&url);
}

