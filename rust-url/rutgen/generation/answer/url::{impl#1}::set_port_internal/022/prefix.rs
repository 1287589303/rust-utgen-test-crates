// Answer 0

#[test]
fn test_set_port_internal_none_none() {
    let mut url = Url {
        serialization: String::from("http://example.com"),
        scheme_end: 4,
        username_end: 0,
        host_start: 7,
        host_end: 18,
        host: HostInternal::Domain,
        port: None,
        path_start: 18,
        query_start: None,
        fragment_start: None,
    };
    url.set_port_internal(None);
}

#[test]
fn test_set_port_internal_none_none_empty_serialization() {
    let mut url = Url {
        serialization: String::new(),
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
    url.set_port_internal(None);
}

#[test]
fn test_set_port_internal_none_none_long_host() {
    let mut url = Url {
        serialization: String::from("http://example.com/path"),
        scheme_end: 4,
        username_end: 0,
        host_start: 7,
        host_end: 18,
        host: HostInternal::Domain,
        port: None,
        path_start: 18,
        query_start: None,
        fragment_start: None,
    };
    url.set_port_internal(None);
}

