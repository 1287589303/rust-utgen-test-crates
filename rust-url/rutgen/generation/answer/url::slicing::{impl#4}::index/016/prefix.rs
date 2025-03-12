// Answer 0

#[test]
fn test_index_after_port_with_valid_port() {
    let url = Url {
        serialization: String::from("http://example.com:8080/path"),
        scheme_end: 4,
        username_end: 4,
        host_start: 12,
        host_end: 23,
        host: HostInternal::Domain,
        port: Some(8080),
        path_start: 27,
        query_start: None,
        fragment_start: None,
    };

    let position = Position::AfterPort;
    let _ = url.index(position);
}

#[test]
fn test_index_after_port_with_min_port() {
    let url = Url {
        serialization: String::from("http://example.com:1/path"),
        scheme_end: 4,
        username_end: 4,
        host_start: 12,
        host_end: 23,
        host: HostInternal::Domain,
        port: Some(1),
        path_start: 27,
        query_start: None,
        fragment_start: None,
    };

    let position = Position::AfterPort;
    let _ = url.index(position);
}

#[test]
fn test_index_after_port_with_max_port() {
    let url = Url {
        serialization: String::from("http://example.com:65535/path"),
        scheme_end: 4,
        username_end: 4,
        host_start: 12,
        host_end: 23,
        host: HostInternal::Domain,
        port: Some(65535),
        path_start: 27,
        query_start: None,
        fragment_start: None,
    };

    let position = Position::AfterPort;
    let _ = url.index(position);
}

