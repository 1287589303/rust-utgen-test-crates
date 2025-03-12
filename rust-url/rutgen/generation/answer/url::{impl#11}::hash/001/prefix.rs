// Answer 0

#[test]
fn test_hash_with_non_empty_serialization() {
    let url = Url {
        serialization: String::from("http://example.com"),
        scheme_end: 4,
        username_end: 0,
        host_start: 7,
        host_end: 11,
        host: HostInternal::Domain,
        port: None,
        path_start: 17,
        query_start: None,
        fragment_start: None,
    };
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    url.hash(&mut hasher);
}

#[test]
fn test_hash_with_non_empty_serialization_another_scheme() {
    let url = Url {
        serialization: String::from("https://example.com/path"),
        scheme_end: 5,
        username_end: 0,
        host_start: 8,
        host_end: 12,
        host: HostInternal::Domain,
        port: None,
        path_start: 17,
        query_start: None,
        fragment_start: None,
    };
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    url.hash(&mut hasher);
}

#[test]
fn test_hash_with_serialization_containing_query_and_fragment() {
    let url = Url {
        serialization: String::from("ftp://user:pass@example.com:21/path?query#fragment"),
        scheme_end: 3,
        username_end: 8,
        host_start: 15,
        host_end: 19,
        host: HostInternal::Domain,
        port: Some(21),
        path_start: 24,
        query_start: Some(29),
        fragment_start: Some(37),
    };
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    url.hash(&mut hasher);
}

#[test]
fn test_hash_with_ip_v4() {
    let url = Url {
        serialization: String::from("http://192.168.1.1"),
        scheme_end: 4,
        username_end: 0,
        host_start: 7,
        host_end: 15,
        host: HostInternal::Ipv4(Ipv4Addr::new(192, 168, 1, 1)),
        port: None,
        path_start: 0,
        query_start: None,
        fragment_start: None,
    };
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    url.hash(&mut hasher);
}

#[test]
fn test_hash_with_ip_v6() {
    let url = Url {
        serialization: String::from("http://[::1]"),
        scheme_end: 4,
        username_end: 0,
        host_start: 7,
        host_end: 11,
        host: HostInternal::Ipv6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)),
        port: None,
        path_start: 0,
        query_start: None,
        fragment_start: None,
    };
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    url.hash(&mut hasher);
}

