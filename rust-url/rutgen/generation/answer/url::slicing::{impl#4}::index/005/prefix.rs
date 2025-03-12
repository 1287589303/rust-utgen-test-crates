// Answer 0

#[test]
fn test_index_after_query_with_fragment() {
    let url = Url {
        serialization: String::from("http://username:password@host:8080/path?query#fragment"),
        scheme_end: 4,
        username_end: 13,
        host_start: 14,
        host_end: 18,
        host: HostInternal::Domain,
        port: Some(8080),
        path_start: 23,
        query_start: Some(29),
        fragment_start: Some(37),
    };
    let position = Position::AfterQuery;
    let result = url.index(position);
}

#[test]
fn test_index_after_query_with_fragment_on_boundary() {
    let url = Url {
        serialization: String::from("https://user:pass@localhost:8000/path?query#"),
        scheme_end: 5,
        username_end: 11,
        host_start: 12,
        host_end: 20,
        host: HostInternal::Domain,
        port: Some(8000),
        path_start: 24,
        query_start: Some(30),
        fragment_start: Some(32),
    };
    let position = Position::AfterQuery;
    let result = url.index(position);
}

#[test]
fn test_index_after_query_with_no_fragment() {
    let url = Url {
        serialization: String::from("ftp://user:pass@server/path?query"),
        scheme_end: 6,
        username_end: 12,
        host_start: 13,
        host_end: 19,
        host: HostInternal::Domain,
        port: None,
        path_start: 23,
        query_start: Some(29),
        fragment_start: None,
    };
    let position = Position::AfterQuery;
    let result = url.index(position);
}

