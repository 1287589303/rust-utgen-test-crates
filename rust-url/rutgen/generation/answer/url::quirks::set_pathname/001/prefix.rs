// Answer 0

#[test]
fn test_set_pathname_cannot_be_base_non_special_scheme() {
    let mut url = Url {
        serialization: "http://example.com".to_string(),
        scheme_end: 4,
        username_end: 0,
        host_start: 7,
        host_end: 14,
        host: HostInternal::None,
        port: None,
        path_start: 0,
        query_start: None,
        fragment_start: None,
    };
    let new_pathname = "path/to/resource";
    set_pathname(&mut url, new_pathname);
}

#[test]
fn test_set_pathname_cannot_be_base_non_special_scheme_empty_path() {
    let mut url = Url {
        serialization: "http://example.com".to_string(),
        scheme_end: 4,
        username_end: 0,
        host_start: 7,
        host_end: 14,
        host: HostInternal::None,
        port: None,
        path_start: 0,
        query_start: None,
        fragment_start: None,
    };
    let new_pathname = "";
    set_pathname(&mut url, new_pathname);
}

#[test]
fn test_set_pathname_cannot_be_base_second_non_special_scheme() {
    let mut url = Url {
        serialization: "ftp://example.com".to_string(),
        scheme_end: 6,
        username_end: 0,
        host_start: 8,
        host_end: 15,
        host: HostInternal::None,
        port: None,
        path_start: 0,
        query_start: None,
        fragment_start: None,
    };
    let new_pathname = "another/path";
    set_pathname(&mut url, new_pathname);
}

