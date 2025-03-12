// Answer 0

#[test]
fn test_partial_cmp_equal_serialization() {
    let url1 = Url {
        serialization: String::from("http://example.com"),
        scheme_end: 4,
        username_end: 0,
        host_start: 7,
        host_end: 15,
        host: HostInternal::Domain,
        port: None,
        path_start: 10,
        query_start: None,
        fragment_start: None,
    };
    
    let url2 = Url {
        serialization: String::from("http://example.com"),
        scheme_end: 4,
        username_end: 0,
        host_start: 7,
        host_end: 15,
        host: HostInternal::Domain,
        port: None,
        path_start: 10,
        query_start: None,
        fragment_start: None,
    };

    url1.partial_cmp(&url2);
}

#[test]
fn test_partial_cmp_different_serialization() {
    let url1 = Url {
        serialization: String::from("http://example.com"),
        scheme_end: 4,
        username_end: 0,
        host_start: 7,
        host_end: 15,
        host: HostInternal::Domain,
        port: None,
        path_start: 10,
        query_start: None,
        fragment_start: None,
    };
    
    let url2 = Url {
        serialization: String::from("https://example.com"),
        scheme_end: 5,
        username_end: 0,
        host_start: 8,
        host_end: 15,
        host: HostInternal::Domain,
        port: None,
        path_start: 10,
        query_start: None,
        fragment_start: None,
    };

    url1.partial_cmp(&url2);
}

#[test]
fn test_partial_cmp_maximal_length() {
    let url1 = Url {
        serialization: String::from(
            "http://a".repeat(1000) + ".com/" + &"a".repeat(1000)),
        scheme_end: 4,
        username_end: 0,
        host_start: 7,
        host_end: 1007,
        host: HostInternal::Domain,
        port: None,
        path_start: 1008,
        query_start: None,
        fragment_start: None,
    };
    
    let url2 = Url {
        serialization: String::from(
            "http://b".repeat(1000) + ".com/" + &"a".repeat(1000)),
        scheme_end: 4,
        username_end: 0,
        host_start: 7,
        host_end: 1007,
        host: HostInternal::Domain,
        port: None,
        path_start: 1008,
        query_start: None,
        fragment_start: None,
    };

    url1.partial_cmp(&url2);
}

#[test]
fn test_partial_cmp_edge_case_serializations() {
    let url1 = Url {
        serialization: String::from("http://localhost"),
        scheme_end: 4,
        username_end: 0,
        host_start: 7,
        host_end: 16,
        host: HostInternal::Domain,
        port: None,
        path_start: 17,
        query_start: None,
        fragment_start: None,
    };
    
    let url2 = Url {
        serialization: String::from("http://127.0.0.1"),
        scheme_end: 4,
        username_end: 0,
        host_start: 7,
        host_end: 15,
        host: HostInternal::Ipv4(Ipv4Addr::new(127, 0, 0, 1)),
        port: None,
        path_start: 16,
        query_start: None,
        fragment_start: None,
    };

    url1.partial_cmp(&url2);
}

