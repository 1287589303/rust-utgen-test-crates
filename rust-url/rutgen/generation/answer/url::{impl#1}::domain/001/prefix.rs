// Answer 0

#[test]
fn test_domain_none_host_none() {
    let url = Url {
        serialization: String::from("http://example.com"),
        scheme_end: 4,
        username_end: 0,
        host_start: 7,
        host_end: 11,
        host: HostInternal::None,
        port: None,
        path_start: 12,
        query_start: None,
        fragment_start: None,
    };
    let result = url.domain();
}

#[test]
fn test_domain_none_host_ipv4() {
    use std::net::Ipv4Addr;  
    let url = Url {
        serialization: String::from("http://192.168.1.1"),
        scheme_end: 4,
        username_end: 0,
        host_start: 7,
        host_end: 15,
        host: HostInternal::Ipv4(Ipv4Addr::new(192, 168, 1, 1)),
        port: None,
        path_start: 16,
        query_start: None,
        fragment_start: None,
    };
    let result = url.domain();
}

#[test]
fn test_domain_none_host_ipv6() {
    use std::net::Ipv6Addr;  
    let url = Url {
        serialization: String::from("http://[::1]"),
        scheme_end: 4,
        username_end: 0,
        host_start: 7,
        host_end: 13,
        host: HostInternal::Ipv6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)),
        port: None,
        path_start: 14,
        query_start: None,
        fragment_start: None,
    };
    let result = url.domain();
}

