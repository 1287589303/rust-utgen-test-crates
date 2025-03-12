// Answer 0

#[test]
fn test_set_host_internal_with_ipv4_host() {
    let mut url = Url {
        serialization: String::from("http://example.com"),
        scheme_end: 4,
        username_end: 4,
        host_start: 4,
        host_end: 4,
        host: HostInternal::None,
        port: None,
        path_start: 0,
        query_start: None,
        fragment_start: None,
    };
    let host = Host::Ipv4(Ipv4Addr::new(192, 168, 1, 1));
    let new_port = Some(Some(8080));
    url.set_host_internal(host, new_port);
}

#[test]
fn test_set_host_internal_with_ipv6_host() {
    let mut url = Url {
        serialization: String::from("ftp://example.com"),
        scheme_end: 4,
        username_end: 4,
        host_start: 4,
        host_end: 4,
        host: HostInternal::None,
        port: None,
        path_start: 0,
        query_start: None,
        fragment_start: None,
    };
    let host = Host::Ipv6(Ipv6Addr::new(0x2001, 0x0db8, 0x85a3, 0x0000, 0x0000, 0x8a2e, 0x0370, 0x7334));
    let new_port = Some(Some(3000));
    url.set_host_internal(host, new_port);
}

#[test]
fn test_set_host_internal_with_dns_host() {
    let mut url = Url {
        serialization: String::from("https://example"),
        scheme_end: 5,
        username_end: 5,
        host_start: 5,
        host_end: 5,
        host: HostInternal::None,
        port: None,
        path_start: 0,
        query_start: None,
        fragment_start: None,
    };
    let host = Host::Domain(String::from("mydomain.com"));
    let new_port = Some(None);
    url.set_host_internal(host, new_port);
}

#[test]
fn test_set_host_internal_with_ipv4_no_port() {
    let mut url = Url {
        serialization: String::from("http://anotherexample.com"),
        scheme_end: 4,
        username_end: 4,
        host_start: 4,
        host_end: 4,
        host: HostInternal::None,
        port: None,
        path_start: 0,
        query_start: None,
        fragment_start: None,
    };
    let host = Host::Ipv4(Ipv4Addr::new(10, 0, 0, 1));
    let new_port = Some(Some(0));
    url.set_host_internal(host, new_port);
}

#[test]
fn test_set_host_internal_with_empty_host() {
    let mut url = Url {
        serialization: String::from("http://somepath"),
        scheme_end: 4,
        username_end: 4,
        host_start: 4,
        host_end: 4,
        host: HostInternal::None,
        port: None,
        path_start: 0,
        query_start: None,
        fragment_start: None,
    };
    let host = Host::Domain(String::from(""));
    let new_port = Some(Some(1234));
    url.set_host_internal(host, new_port);
}

