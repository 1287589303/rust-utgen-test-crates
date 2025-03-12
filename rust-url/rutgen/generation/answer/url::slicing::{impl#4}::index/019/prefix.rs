// Answer 0

#[test]
fn test_index_before_port_with_valid_port() {
    let url = Url {
        serialization: "http://user:password@host:8080/path?query#fragment".to_string(),
        scheme_end: "http".len() as u32,
        username_end: "user:password".len() as u32,
        host_start: "user:password@".len() as u32,
        host_end: "user:password@host".len() as u32,
        host: HostInternal::Domain,
        port: Some(8080),
        path_start: "http://user:password@host:8080/".len() as u32,
        query_start: Some("http://user:password@host:8080/path?query".len() as u32),
        fragment_start: Some("http://user:password@host:8080/path?query#fragment".len() as u32),
    };
    let position = Position::BeforePort;
    let _ = url.index(position);
}

#[test]
fn test_index_before_port_with_max_port_value() {
    let url = Url {
        serialization: "http://user:password@host:65535/path?query#fragment".to_string(),
        scheme_end: "http".len() as u32,
        username_end: "user:password".len() as u32,
        host_start: "user:password@".len() as u32,
        host_end: "user:password@host".len() as u32,
        host: HostInternal::Domain,
        port: Some(65535),
        path_start: "http://user:password@host:65535/".len() as u32,
        query_start: Some("http://user:password@host:65535/path?query".len() as u32),
        fragment_start: Some("http://user:password@host:65535/path?query#fragment".len() as u32),
    };
    let position = Position::BeforePort;
    let _ = url.index(position);
}

