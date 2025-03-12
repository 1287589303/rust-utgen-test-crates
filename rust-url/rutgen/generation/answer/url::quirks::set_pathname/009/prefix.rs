// Answer 0

#[test]
fn test_set_pathname_with_host() {
    let mut url = Url {
        serialization: "http://example.com".to_string(),
        scheme_end: 4,
        username_end: 0,
        host_start: 7,
        host_end: 17,
        host: HostInternal::Domain("example.com".to_string()),
        port: None,
        path_start: 18,
        query_start: None,
        fragment_start: None,
    };

    let new_pathname = ""; // new_pathname.is_empty() is true

    set_pathname(&mut url, new_pathname);
}

#[test]
fn test_set_pathname_with_host_and_path() {
    let mut url = Url {
        serialization: "http://example.com/path".to_string(),
        scheme_end: 4,
        username_end: 0,
        host_start: 7,
        host_end: 17,
        host: HostInternal::Domain("example.com".to_string()),
        port: None,
        path_start: 18,
        query_start: None,
        fragment_start: None,
    };

    let new_pathname = "another/path"; // starts with neither '/' nor '\\', and is not empty

    set_pathname(&mut url, new_pathname);
}

#[test]
fn test_set_pathname_with_idempotence() {
    let mut url = Url {
        serialization: "http://example.com/some/path".to_string(),
        scheme_end: 4,
        username_end: 0,
        host_start: 7,
        host_end: 17,
        host: HostInternal::Domain("example.com".to_string()),
        port: None,
        path_start: 18,
        query_start: None,
        fragment_start: None,
    };

    let new_pathname = ""; // new_pathname.is_empty() is true

    set_pathname(&mut url, new_pathname);
    set_pathname(&mut url, new_pathname); // should have no effect after first call
}

