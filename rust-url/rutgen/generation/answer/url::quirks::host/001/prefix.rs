// Answer 0

#[test]
fn test_host_http_valid() {
    let url = Url {
        serialization: String::from("http://example.com"),
        scheme_end: 4,
        username_end: 0,
        host_start: 7,
        host_end: 18,
        host: HostInternal::new("example.com", None),
        port: None,
        path_start: 0,
        query_start: None,
        fragment_start: None,
    };
    let _ = host(&url);
}

#[test]
fn test_host_https_valid() {
    let url = Url {
        serialization: String::from("https://example.com"),
        scheme_end: 5,
        username_end: 0,
        host_start: 8,
        host_end: 18,
        host: HostInternal::new("example.com", None),
        port: None,
        path_start: 0,
        query_start: None,
        fragment_start: None,
    };
    let _ = host(&url);
}

#[test]
fn test_host_ftp_with_user_pass() {
    let url = Url {
        serialization: String::from("ftp://user:pass@example.com:21/path"),
        scheme_end: 3,
        username_end: 10,
        host_start: 14,
        host_end: 18,
        host: HostInternal::new("example.com", Some(21)),
        port: Some(21),
        path_start: 22,
        query_start: None,
        fragment_start: None,
    };
    let _ = host(&url);
}

#[test]
fn test_host_with_port() {
    let url = Url {
        serialization: String::from("http://example.com:8080"),
        scheme_end: 4,
        username_end: 0,
        host_start: 7,
        host_end: 18,
        host: HostInternal::new("example.com", Some(8080)),
        port: Some(8080),
        path_start: 0,
        query_start: None,
        fragment_start: None,
    };
    let _ = host(&url);
}

#[test]
fn test_host_non_hierarchical_path() {
    let url = Url {
        serialization: String::from("test.com"),
        scheme_end: 0,
        username_end: 0,
        host_start: 0,
        host_end: 8,
        host: HostInternal::new("test.com", None),
        port: None,
        path_start: 0,
        query_start: None,
        fragment_start: None,
    };
    let _ = host(&url);
}

#[test]
#[should_panic]
fn test_host_empty_host() {
    let url = Url {
        serialization: String::from("http:///"),
        scheme_end: 4,
        username_end: 0,
        host_start: 7,
        host_end: 7,
        host: HostInternal::new("", None),
        port: None,
        path_start: 0,
        query_start: None,
        fragment_start: None,
    };
    let _ = host(&url);
}

#[test]
fn test_host_with_query_and_fragment() {
    let url = Url {
        serialization: String::from("http://example.com/path?query#fragment"),
        scheme_end: 4,
        username_end: 0,
        host_start: 7,
        host_end: 18,
        host: HostInternal::new("example.com", None),
        port: None,
        path_start: 22,
        query_start: Some(23),
        fragment_start: Some(30),
    };
    let _ = host(&url);
}

