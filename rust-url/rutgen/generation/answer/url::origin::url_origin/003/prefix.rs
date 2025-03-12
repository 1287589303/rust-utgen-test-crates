// Answer 0

#[test]
fn test_url_origin_ftp_scheme() {
    let url = Url {
        serialization: String::from("ftp://example.com:21"),
        scheme_end: 3,
        username_end: 0,
        host_start: 6,
        host_end: 16,
        host: Host::Domain(String::from("example.com")),
        port: Some(21),
        path_start: 0,
        query_start: None,
        fragment_start: None,
    };
    let _ = url_origin(&url);
}

#[test]
fn test_url_origin_https_scheme() {
    let url = Url {
        serialization: String::from("https://example.com:443"),
        scheme_end: 5,
        username_end: 0,
        host_start: 8,
        host_end: 18,
        host: Host::Domain(String::from("example.com")),
        port: Some(443),
        path_start: 0,
        query_start: None,
        fragment_start: None,
    };
    let _ = url_origin(&url);
}

#[test]
fn test_url_origin_http_scheme() {
    let url = Url {
        serialization: String::from("http://example.net:80"),
        scheme_end: 4,
        username_end: 0,
        host_start: 7,
        host_end: 17,
        host: Host::Domain(String::from("example.net")),
        port: Some(80),
        path_start: 0,
        query_start: None,
        fragment_start: None,
    };
    let _ = url_origin(&url);
}

#[test]
fn test_url_origin_ws_scheme() {
    let url = Url {
        serialization: String::from("ws://example.org:8080"),
        scheme_end: 2,
        username_end: 0,
        host_start: 5,
        host_end: 15,
        host: Host::Domain(String::from("example.org")),
        port: Some(8080),
        path_start: 0,
        query_start: None,
        fragment_start: None,
    };
    let _ = url_origin(&url);
}

#[test]
fn test_url_origin_wss_scheme() {
    let url = Url {
        serialization: String::from("wss://example.com:8443"),
        scheme_end: 4,
        username_end: 0,
        host_start: 7,
        host_end: 17,
        host: Host::Domain(String::from("example.com")),
        port: Some(8443),
        path_start: 0,
        query_start: None,
        fragment_start: None,
    };
    let _ = url_origin(&url);
}

