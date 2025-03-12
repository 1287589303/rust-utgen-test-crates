// Answer 0

#[test]
fn test_url_origin_with_ws_scheme() {
    let url = Url {
        serialization: String::from("ws://example.com:8080"),
        scheme_end: 2,
        username_end: 0,
        host_start: 5,
        host_end: 16,
        host: Host::Domain(String::from("example.com")),
        port: Some(8080),
        path_start: 0,
        query_start: None,
        fragment_start: None,
    };
    let _ = url_origin(&url);
}

#[test]
fn test_url_origin_with_http_scheme() {
    let url = Url {
        serialization: String::from("http://example.com:80"),
        scheme_end: 4,
        username_end: 0,
        host_start: 7,
        host_end: 18,
        host: Host::Domain(String::from("example.com")),
        port: Some(80),
        path_start: 0,
        query_start: None,
        fragment_start: None,
    };
    let _ = url_origin(&url);
}

#[test]
fn test_url_origin_with_https_scheme() {
    let url = Url {
        serialization: String::from("https://example.com:443"),
        scheme_end: 5,
        username_end: 0,
        host_start: 6,
        host_end: 17,
        host: Host::Domain(String::from("example.com")),
        port: Some(443),
        path_start: 0,
        query_start: None,
        fragment_start: None,
    };
    let _ = url_origin(&url);
}

#[test]
fn test_url_origin_with_ftp_scheme() {
    let url = Url {
        serialization: String::from("ftp://ftp.example.com:21"),
        scheme_end: 3,
        username_end: 0,
        host_start: 6,
        host_end: 20,
        host: Host::Domain(String::from("ftp.example.com")),
        port: Some(21),
        path_start: 0,
        query_start: None,
        fragment_start: None,
    };
    let _ = url_origin(&url);
}

#[test]
fn test_url_origin_with_wss_scheme() {
    let url = Url {
        serialization: String::from("wss://secure.example.com:8443"),
        scheme_end: 4,
        username_end: 0,
        host_start: 5,
        host_end: 23,
        host: Host::Domain(String::from("secure.example.com")),
        port: Some(8443),
        path_start: 0,
        query_start: None,
        fragment_start: None,
    };
    let _ = url_origin(&url);
}

