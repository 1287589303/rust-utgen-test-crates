// Answer 0

#[test]
fn test_index_before_fragment_with_valid_fragment_start() {
    let url = Url {
        serialization: String::from("http://example.com/path?query#fragment"),
        scheme_end: 4,
        username_end: 4,
        host_start: 7,
        host_end: 20,
        host: HostInternal::Domain,
        port: None,
        path_start: 21,
        query_start: Some(27),
        fragment_start: Some(35),
    };
    let position = Position::BeforeFragment;
    let _ = url.index(position);
}

#[test]
fn test_index_before_fragment_with_edge_case_fragment_start() {
    let url = Url {
        serialization: String::from("http://example.com/#"),
        scheme_end: 4,
        username_end: 4,
        host_start: 7,
        host_end: 20,
        host: HostInternal::Domain,
        port: None,
        path_start: 21,
        query_start: None,
        fragment_start: Some(22),
    };
    let position = Position::BeforeFragment;
    let _ = url.index(position);
}

#[test]
fn test_index_before_fragment_with_empty_fragment() {
    let url = Url {
        serialization: String::from("http://example.com/path#"),
        scheme_end: 4,
        username_end: 4,
        host_start: 7,
        host_end: 20,
        host: HostInternal::Domain,
        port: None,
        path_start: 21,
        query_start: None,
        fragment_start: Some(22),
    };
    let position = Position::BeforeFragment;
    let _ = url.index(position);
}

