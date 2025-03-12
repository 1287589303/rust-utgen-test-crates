// Answer 0

#[test]
fn test_strip_trailing_spaces_non_hierarchical_with_spaces() {
    let mut url = Url {
        serialization: "example/path    ".to_string(),
        scheme_end: 7,
        username_end: 0,
        host_start: 0,
        host_end: 0,
        host: HostInternal::None,
        port: None,
        path_start: 0,
        query_start: Some(0),
        fragment_start: None,
    };
    url.strip_trailing_spaces_from_opaque_path();
}

#[test]
fn test_strip_trailing_spaces_non_hierarchical_multiple_spaces() {
    let mut url = Url {
        serialization: "another/path     ".to_string(),
        scheme_end: 7,
        username_end: 0,
        host_start: 0,
        host_end: 0,
        host: HostInternal::None,
        port: None,
        path_start: 0,
        query_start: Some(0),
        fragment_start: None,
    };
    url.strip_trailing_spaces_from_opaque_path();
}

#[test]
fn test_strip_trailing_spaces_non_hierarchical_no_spaces() {
    let mut url = Url {
        serialization: "non_hierarchical_path".to_string(),
        scheme_end: 22,
        username_end: 0,
        host_start: 0,
        host_end: 0,
        host: HostInternal::None,
        port: None,
        path_start: 0,
        query_start: Some(0),
        fragment_start: None,
    };
    url.strip_trailing_spaces_from_opaque_path();
}

