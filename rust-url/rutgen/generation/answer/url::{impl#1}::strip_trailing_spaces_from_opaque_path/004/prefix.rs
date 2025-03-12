// Answer 0

#[test]
fn test_strip_trailing_spaces_opaque_path_not_base() {
    let mut url = Url {
        serialization: String::from("validopaquePath   "),
        scheme_end: 0,
        username_end: 0,
        host_start: 0,
        host_end: 0,
        host: HostInternal::None,
        port: None,
        path_start: 0,
        query_start: None,
        fragment_start: None,
    };
    url.strip_trailing_spaces_from_opaque_path();
}

#[test]
fn test_strip_trailing_spaces_opaque_path_with_valid_segment() {
    let mut url = Url {
        serialization: String::from("justAnotherValidOpaquePath   "),
        scheme_end: 0,
        username_end: 0,
        host_start: 0,
        host_end: 0,
        host: HostInternal::None,
        port: None,
        path_start: 0,
        query_start: None,
        fragment_start: None,
    };
    url.strip_trailing_spaces_from_opaque_path();
}

#[test]
fn test_strip_trailing_spaces_opaque_path_with_mixed_cases() {
    let mut url = Url {
        serialization: String::from("TestPathWithSpaces   "),
        scheme_end: 0,
        username_end: 0,
        host_start: 0,
        host_end: 0,
        host: HostInternal::None,
        port: None,
        path_start: 0,
        query_start: None,
        fragment_start: None,
    };
    url.strip_trailing_spaces_from_opaque_path();
}

#[test]
fn test_strip_trailing_spaces_opaque_path_with_only_spaces() {
    let mut url = Url {
        serialization: String::from("    "),
        scheme_end: 0,
        username_end: 0,
        host_start: 0,
        host_end: 0,
        host: HostInternal::None,
        port: None,
        path_start: 0,
        query_start: None,
        fragment_start: None,
    };
    url.strip_trailing_spaces_from_opaque_path();
}

