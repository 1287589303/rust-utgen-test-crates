// Answer 0

#[test]
fn test_strip_trailing_spaces_from_opaque_path_with_trailing_spaces() {
    let mut url = Url {
        serialization: "example://path_with_spaces   ".to_owned(),
        scheme_end: 7,
        username_end: 0,
        host_start: 0,
        host_end: 0,
        host: HostInternal::None,
        port: None,
        path_start: 0,
        query_start: None,
        fragment_start: Some(0),
    };
    url.strip_trailing_spaces_from_opaque_path();
}

#[test]
fn test_strip_trailing_spaces_from_opaque_path_with_only_spaces() {
    let mut url = Url {
        serialization: "   ".to_owned(),
        scheme_end: 0,
        username_end: 0,
        host_start: 0,
        host_end: 0,
        host: HostInternal::None,
        port: None,
        path_start: 0,
        query_start: None,
        fragment_start: Some(0),
    };
    url.strip_trailing_spaces_from_opaque_path();
}

#[test]
fn test_strip_trailing_spaces_from_opaque_path_empty_string() {
    let mut url = Url {
        serialization: "".to_owned(),
        scheme_end: 0,
        username_end: 0,
        host_start: 0,
        host_end: 0,
        host: HostInternal::None,
        port: None,
        path_start: 0,
        query_start: None,
        fragment_start: Some(0),
    };
    url.strip_trailing_spaces_from_opaque_path();
}

#[test]
fn test_strip_trailing_spaces_from_opaque_path_no_trailing_spaces() {
    let mut url = Url {
        serialization: "example://path_without_spaces".to_owned(),
        scheme_end: 7,
        username_end: 0,
        host_start: 0,
        host_end: 0,
        host: HostInternal::None,
        port: None,
        path_start: 0,
        query_start: None,
        fragment_start: Some(0),
    };
    url.strip_trailing_spaces_from_opaque_path();
}

#[test]
fn test_strip_trailing_spaces_from_opaque_path_long_string() {
    let mut url = Url {
        serialization: "x".repeat(1000) + "    ",
        scheme_end: 1000,
        username_end: 0,
        host_start: 0,
        host_end: 0,
        host: HostInternal::None,
        port: None,
        path_start: 0,
        query_start: None,
        fragment_start: Some(0),
    };
    url.strip_trailing_spaces_from_opaque_path();
}

