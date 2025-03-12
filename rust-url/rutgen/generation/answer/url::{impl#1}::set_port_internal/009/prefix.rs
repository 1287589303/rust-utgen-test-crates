// Answer 0

#[test]
fn test_set_port_internal_with_same_port() {
    let mut url = Url {
        serialization: String::from("http://example.com:8080/path"),
        scheme_end: 4,
        username_end: 0,
        host_start: 7,
        host_end: 21,
        host: HostInternal::Domain,
        port: Some(8080),
        path_start: 25,
        query_start: None,
        fragment_start: None,
    };

    url.set_port_internal(Some(8080));
}

#[test]
fn test_set_port_internal_with_boundary_port() {
    let mut url = Url {
        serialization: String::from("http://example.com:0/path"),
        scheme_end: 4,
        username_end: 0,
        host_start: 7,
        host_end: 23,
        host: HostInternal::Domain,
        port: Some(0),
        path_start: 27,
        query_start: None,
        fragment_start: None,
    };

    url.set_port_internal(Some(0));
}

#[test]
fn test_set_port_internal_with_max_port() {
    let mut url = Url {
        serialization: String::from("http://example.com:65535/path"),
        scheme_end: 4,
        username_end: 0,
        host_start: 7,
        host_end: 24,
        host: HostInternal::Domain,
        port: Some(65535),
        path_start: 29,
        query_start: None,
        fragment_start: None,
    };

    url.set_port_internal(Some(65535));
}

