// Answer 0

#[test]
fn test_set_search_with_query() {
    let mut url = Url {
        serialization: String::from("http://example.com"),
        scheme_end: 4,
        username_end: 0,
        host_start: 7,
        host_end: 18,
        host: HostInternal::from_str("example.com").unwrap(),
        port: None,
        path_start: 0,
        query_start: None,
        fragment_start: None,
    };
    set_search(&mut url, "?query=param");
}

#[test]
fn test_set_search_without_question_mark() {
    let mut url = Url {
        serialization: String::from("http://example.com"),
        scheme_end: 4,
        username_end: 0,
        host_start: 7,
        host_end: 18,
        host: HostInternal::from_str("example.com").unwrap(),
        port: None,
        path_start: 0,
        query_start: None,
        fragment_start: None,
    };
    set_search(&mut url, "query=param");
}

#[test]
fn test_set_search_empty_string() {
    let mut url = Url {
        serialization: String::from("http://example.com"),
        scheme_end: 4,
        username_end: 0,
        host_start: 7,
        host_end: 18,
        host: HostInternal::from_str("example.com").unwrap(),
        port: None,
        path_start: 0,
        query_start: None,
        fragment_start: None,
    };
    set_search(&mut url, "?");
}

