// Answer 0

#[test]
fn test_domain_with_valid_domain() {
    let url = Url {
        serialization: String::from("https://example.com/"),
        scheme_end: 4,
        username_end: 0,
        host_start: 8,
        host_end: 19,
        host: HostInternal::Domain,
        port: None,
        path_start: 20,
        query_start: None,
        fragment_start: None,
    };
    let _ = url.domain();
}

#[test]
fn test_domain_with_subdomain() {
    let url = Url {
        serialization: String::from("https://sub.example.com/path"),
        scheme_end: 4,
        username_end: 0,
        host_start: 8,
        host_end: 20,
        host: HostInternal::Domain,
        port: None,
        path_start: 21,
        query_start: None,
        fragment_start: None,
    };
    let _ = url.domain();
}

#[test]
fn test_domain_with_ipv4_host() {
    let url = Url {
        serialization: String::from("https://127.0.0.1/"),
        scheme_end: 4,
        username_end: 0,
        host_start: 8,
        host_end: 14,
        host: HostInternal::Ipv4(Ipv4Addr::new(127, 0, 0, 1)),
        port: None,
        path_start: 15,
        query_start: None,
        fragment_start: None,
    };
    let _ = url.domain();
}

#[test]
fn test_domain_with_none_host() {
    let url = Url {
        serialization: String::from("https:///"),
        scheme_end: 4,
        username_end: 0,
        host_start: 8,
        host_end: 8,
        host: HostInternal::None,
        port: None,
        path_start: 8,
        query_start: None,
        fragment_start: None,
    };
    let _ = url.domain();
}

