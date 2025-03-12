// Answer 0

#[test]
fn test_set_host_valid_host_with_port() {
    let mut url = Url {
        serialization: "http://example.com".to_string(),
        scheme_end: 4,
        username_end: 0,
        host_start: 7,
        host_end: 18,
        host: Host::Domain("example.com".to_string()),
        port: None,
        path_start: 18,
        query_start: None,
        fragment_start: None,
    };

    let new_host = "example.org:8080";
    let result = set_host(&mut url, new_host);
}

#[test]
fn test_set_host_valid_opaque_host_with_port() {
    let mut url = Url {
        serialization: "http://example.com".to_string(),
        scheme_end: 4,
        username_end: 0,
        host_start: 7,
        host_end: 18,
        host: Host::Domain("example.com".to_string()),
        port: None,
        path_start: 18,
        query_start: None,
        fragment_start: None,
    };

    let new_host = "opaque_host:3000";
    let result = set_host(&mut url, new_host);
}

#[test]
fn test_set_host_valid_host_with_port_no_username() {
    let mut url = Url {
        serialization: "http://example.com".to_string(),
        scheme_end: 4,
        username_end: 0,
        host_start: 7,
        host_end: 18,
        host: Host::Domain("example.com".to_string()),
        port: None,
        path_start: 18,
        query_start: None,
        fragment_start: None,
    };

    let new_host = "test.com:4000";
    let result = set_host(&mut url, new_host);
}

