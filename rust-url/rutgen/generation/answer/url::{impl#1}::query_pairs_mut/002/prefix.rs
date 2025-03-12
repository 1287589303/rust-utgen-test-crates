// Answer 0

#[test]
fn test_query_pairs_mut_empty_initialization() {
    let mut url = Url {
        serialization: String::from("https://example.net"),
        scheme_end: 5,
        username_end: 0,
        host_start: 0,
        host_end: 11,
        host: HostInternal::Domain,
        port: None,
        path_start: 0,
        query_start: None,
        fragment_start: None,
    };
    let _ = url.query_pairs_mut();
}

#[test]
fn test_query_pairs_mut_invalid_start() {
    let mut url = Url {
        serialization: String::from("https://example.net/some/path"),
        scheme_end: 5,
        username_end: 0,
        host_start: 0,
        host_end: 11,
        host: HostInternal::Domain,
        port: None,
        path_start: 23,
        query_start: None,
        fragment_start: None,
    };
    let _ = url.query_pairs_mut();
}

