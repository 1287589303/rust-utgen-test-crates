// Answer 0

#[test]
fn test_index_before_port_with_port_present() {
    let url = Url {
        serialization: String::from("http://user:pass@localhost:8080/path?query#fragment"),
        scheme_end: 4,
        username_end: 8,
        host_start: 9,
        host_end: 15, // host_end is not ':' character
        host: HostInternal::Domain,
        port: Some(8080),
        path_start: 19,
        query_start: Some(24),
        fragment_start: Some(30),
    };
    let position = Position::BeforePort;
    let result = url.index(position);
}

#[test]
fn test_index_before_port_with_port_present_edge_case() {
    let url = Url {
        serialization: String::from("http://user:pass@localhost:65535/path?query#fragment"),
        scheme_end: 4,
        username_end: 8,
        host_start: 9,
        host_end: 15, // host_end is not ':' character
        host: HostInternal::Domain,
        port: Some(65535),
        path_start: 19,
        query_start: Some(24),
        fragment_start: Some(30),
    };
    let position = Position::BeforePort;
    let result = url.index(position);
}

