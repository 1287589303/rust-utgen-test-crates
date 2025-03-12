// Answer 0

#[test]
fn test_index_before_fragment_no_fragment() {
    let url = Url {
        serialization: String::from("http://example.com"),
        scheme_end: 4,
        username_end: 4,
        host_start: 7,
        host_end: 17,
        host: HostInternal::Domain,
        port: None,
        path_start: 17,
        query_start: None,
        fragment_start: None,
    };

    let position = Position::BeforeFragment;
    let _result = url.index(position);
}

#[test]
fn test_index_before_fragment_no_fragment_with_path() {
    let url = Url {
        serialization: String::from("https://user:pass@example.com/path"),
        scheme_end: 5,
        username_end: 10,
        host_start: 13,
        host_end: 23,
        host: HostInternal::Domain,
        port: None,
        path_start: 23,
        query_start: None,
        fragment_start: None,
    };

    let position = Position::BeforeFragment;
    let _result = url.index(position);
}

#[test]
fn test_index_before_fragment_no_fragment_with_query() {
    let url = Url {
        serialization: String::from("ftp://example.com/path?query=1"),
        scheme_end: 6,
        username_end: 6,
        host_start: 10,
        host_end: 20,
        host: HostInternal::Domain,
        port: None,
        path_start: 20,
        query_start: Some(24),
        fragment_start: None,
    };

    let position = Position::BeforeFragment;
    let _result = url.index(position);
}

