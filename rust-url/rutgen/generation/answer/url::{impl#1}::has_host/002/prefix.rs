// Answer 0

#[test]
fn test_has_host_none() {
    let url = Url {
        serialization: String::from(""),
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
    url.has_host();
}

#[test]
fn test_has_host_domain() {
    let url = Url {
        serialization: String::from("http://example.com"),
        scheme_end: 4,
        username_end: 0,
        host_start: 7,
        host_end: 18,
        host: HostInternal::Domain,
        port: None,
        path_start: 19,
        query_start: None,
        fragment_start: None,
    };
    url.has_host();
}

#[test]
fn test_has_host_ipv4() {
    let url = Url {
        serialization: String::from("http://192.168.1.1"),
        scheme_end: 4,
        username_end: 0,
        host_start: 7,
        host_end: 17,
        host: HostInternal::Ipv4(Ipv4Addr::new(192, 168, 1, 1)),
        port: None,
        path_start: 18,
        query_start: None,
        fragment_start: None,
    };
    url.has_host();
}

#[test]
fn test_has_host_ipv6() {
    let url = Url {
        serialization: String::from("http://[::1]"),
        scheme_end: 4,
        username_end: 0,
        host_start: 7,
        host_end: 12,
        host: HostInternal::Ipv6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)),
        port: None,
        path_start: 13,
        query_start: None,
        fragment_start: None,
    };
    url.has_host();
}

