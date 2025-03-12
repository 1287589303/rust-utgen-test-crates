// Answer 0

#[test]
fn test_url_origin_with_invalid_scheme() {
    let url = Url {
        serialization: String::from("ftp://example.com"),
        scheme_end: 3,
        username_end: 0,
        host_start: 6,
        host_end: 18,
        host: Host::Domain(String::from("example.com")),
        port: Some(21),
        path_start: 0,
        query_start: None,
        fragment_start: None,
    };
    let _origin = url_origin(&url);
}

#[test]
fn test_url_origin_with_non_special_scheme() {
    let url = Url {
        serialization: String::from("invalidscheme://example.com"),
        scheme_end: 14,
        username_end: 0,
        host_start: 13,
        host_end: 25,
        host: Host::Domain(String::from("example.com")),
        port: None,
        path_start: 0,
        query_start: None,
        fragment_start: None,
    };
    let _origin = url_origin(&url);
}

#[test]
fn test_url_origin_with_empty_scheme() {
    let url = Url {
        serialization: String::from("://example.com"),
        scheme_end: 0,
        username_end: 0,
        host_start: 3,
        host_end: 15,
        host: Host::Domain(String::from("example.com")),
        port: None,
        path_start: 0,
        query_start: None,
        fragment_start: None,
    };
    let _origin = url_origin(&url);
}

#[test]
fn test_url_origin_with_invalid_characters_in_scheme() {
    let url = Url {
        serialization: String::from("inv@lid://example.com"),
        scheme_end: 8,
        username_end: 0,
        host_start: 9,
        host_end: 21,
        host: Host::Domain(String::from("example.com")),
        port: None,
        path_start: 0,
        query_start: None,
        fragment_start: None,
    };
    let _origin = url_origin(&url);
}

