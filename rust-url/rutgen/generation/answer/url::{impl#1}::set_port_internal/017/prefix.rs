// Answer 0

#[test]
fn test_set_port_internal_case1() {
    let mut url = Url {
        serialization: "http://example.com:65535/path".to_string(),
        scheme_end: 4,
        username_end: 0,
        host_start: 7,
        host_end: 22,
        host: HostInternal::Domain,
        port: Some(u16::MAX),
        path_start: 27,
        query_start: Some(10),
        fragment_start: Some(20),
    };
    url.set_port_internal(None);
}

#[test]
fn test_set_port_internal_case2() {
    let mut url = Url {
        serialization: "http://example.com:65535/path?query=value#fragment".to_string(),
        scheme_end: 4,
        username_end: 0,
        host_start: 7,
        host_end: 22,
        host: HostInternal::Domain,
        port: Some(u16::MAX),
        path_start: 27,
        query_start: Some(10),
        fragment_start: Some(20),
    };
    url.set_port_internal(None);
}

