// Answer 0

#[test]
fn test_set_host_with_empty_host_and_username() {
    let mut url = Url {
        serialization: "http://user:pass@valid.com:8080/path".to_string(),
        scheme_end: 4,
        username_end: 10,
        host_start: 7,
        host_end: 16,
        host: Host::Domain("valid.com".to_string()),
        port: Some(8080),
        path_start: 20,
        query_start: None,
        fragment_start: None,
    };
    let new_host = "localhost:3000";
    let result = set_host(&mut url, new_host);
    // result is expected to be Err(()) since opt_port should match Some(Some(_)) with an empty host
}

#[test]
fn test_set_host_with_non_empty_host_and_username() {
    let mut url = Url {
        serialization: "http://user:pass@valid.com:8080/path".to_string(),
        scheme_end: 4,
        username_end: 10,
        host_start: 7,
        host_end: 16,
        host: Host::Domain("valid.com".to_string()),
        port: Some(8080),
        path_start: 20,
        query_start: None,
        fragment_start: None,
    };
    let new_host = "new.hostname:3000";
    let result = set_host(&mut url, new_host);
    // result should be Ok(()) since hostname is non-empty but keep username and port
}

