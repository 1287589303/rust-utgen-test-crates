// Answer 0

#[test]
fn test_query_none_fragment_some() {
    let url = Url {
        serialization: String::from("https://example.com/products"),
        scheme_end: 5,
        username_end: 0,
        host_start: 0,
        host_end: 0,
        host: HostInternal::None,
        port: None,
        path_start: 0,
        query_start: None,
        fragment_start: Some(0),
    };
    let query = url.query();
    query;
}

#[test]
fn test_query_none_fragment_none() {
    let url = Url {
        serialization: String::from("https://example.com/products"),
        scheme_end: 5,
        username_end: 0,
        host_start: 0,
        host_end: 0,
        host: HostInternal::None,
        port: None,
        path_start: 0,
        query_start: None,
        fragment_start: None,
    };
    let query = url.query();
    query;
}

#[test]
fn test_query_none_fragment_some_non_zero() {
    let url = Url {
        serialization: String::from("https://example.com/products"),
        scheme_end: 5,
        username_end: 0,
        host_start: 0,
        host_end: 0,
        host: HostInternal::None,
        port: None,
        path_start: 0,
        query_start: None,
        fragment_start: Some(123),
    };
    let query = url.query();
    query;
}

