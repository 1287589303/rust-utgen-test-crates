// Answer 0

#[test]
fn test_query_with_fragment() {
    let serialization = "https://example.com/products?page=2#fragment".to_string();
    let url = Url {
        serialization,
        scheme_end: 5,
        username_end: 0,
        host_start: 0,
        host_end: 0,
        host: HostInternal::None,
        port: None,
        path_start: 30,
        query_start: Some(30),
        fragment_start: Some(37),
    };
    let _ = url.query();
}

#[test]
fn test_query_with_no_fragment() {
    let serialization = "https://example.com/products?page=2".to_string();
    let url = Url {
        serialization,
        scheme_end: 5,
        username_end: 0,
        host_start: 0,
        host_end: 0,
        host: HostInternal::None,
        port: None,
        path_start: 30,
        query_start: Some(30),
        fragment_start: None,
    };
    let _ = url.query();
}

#[test]
fn test_query_with_special_characters() {
    let serialization = "https://example.com/?country=español#somefragment".to_string();
    let url = Url {
        serialization,
        scheme_end: 5,
        username_end: 0,
        host_start: 0,
        host_end: 0,
        host: HostInternal::None,
        port: None,
        path_start: 30,
        query_start: Some(30),
        fragment_start: Some(50),
    };
    let _ = url.query();
}

