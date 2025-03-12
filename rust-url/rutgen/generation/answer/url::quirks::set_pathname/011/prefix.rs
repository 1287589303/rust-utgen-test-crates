// Answer 0

#[test]
fn test_set_pathname_regular_case() {
    let mut url = Url {
        serialization: String::from("http://example.com"),
        scheme_end: 4,
        username_end: 0,
        host_start: 7,
        host_end: 18,
        host: HostInternal::Domain(String::from("example.com")),
        port: None,
        path_start: 0,
        query_start: None,
        fragment_start: None,
    };
    let new_pathname = "path/to/resource";
    set_pathname(&mut url, new_pathname);
}

#[test]
fn test_set_pathname_with_special_character() {
    let mut url = Url {
        serialization: String::from("ftp://example.com"),
        scheme_end: 4,
        username_end: 0,
        host_start: 7,
        host_end: 18,
        host: HostInternal::Domain(String::from("example.com")),
        port: None,
        path_start: 0,
        query_start: None,
        fragment_start: None,
    };
    let new_pathname = "another/resource";
    set_pathname(&mut url, new_pathname);
}

#[test]
fn test_set_pathname_with_empty_host() {
    let mut url = Url {
        serialization: String::from("http://localhost"),
        scheme_end: 4,
        username_end: 0,
        host_start: 7,
        host_end: 18,
        host: HostInternal::Domain(String::from("localhost")),
        port: Some(8080),
        path_start: 0,
        query_start: None,
        fragment_start: None,
    };
    let new_pathname = "non-empty-path";
    set_pathname(&mut url, new_pathname);
}

