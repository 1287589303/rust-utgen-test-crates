// Answer 0

#[test]
fn test_index_before_host_valid() {
    let url = Url {
        serialization: String::from("http://user:pass@host:8080/path?query#fragment"),
        scheme_end: 4,
        username_end: 8,
        host_start: 13,
        host_end: 17,
        host: HostInternal::Domain,
        port: Some(8080),
        path_start: 22,
        query_start: Some(27),
        fragment_start: Some(33),
    };
    let position = Position::BeforeHost;
    let _ = url.index(position);
}

#[test]
fn test_index_before_host_boundary() {
    let url = Url {
        serialization: String::from("ftp://host"),
        scheme_end: 5,
        username_end: 5,
        host_start: 6,
        host_end: 10,
        host: HostInternal::Domain,
        port: None,
        path_start: 10,
        query_start: None,
        fragment_start: None,
    };
    let position = Position::BeforeHost;
    let _ = url.index(position);
}

#[test]
#[should_panic]
fn test_index_before_host_invalid() {
    let url = Url {
        serialization: String::from("invalidurl"),
        scheme_end: 0,
        username_end: 0,
        host_start: 0,
        host_end: 0,
        host: HostInternal::None,
        port: None,
        path_start: 0,
        query_start: None,
        fragment_start: None,
    };
    let position = Position::BeforeHost; 
    let _ = url.index(position);
}

