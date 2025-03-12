// Answer 0

#[test]
fn test_host_with_valid_domain() {
    let mut url = Url {
        serialization: "https://example.com".to_string(),
        scheme_end: 5,
        username_end: 0,
        host_start: 8,
        host_end: 19,
        host: HostInternal::Domain,
        port: None,
        path_start: 20,
        query_start: None,
        fragment_start: None,
    };
    let _ = url.host();
}

#[test]
fn test_host_with_another_valid_domain() {
    let mut url = Url {
        serialization: "http://subdomain.example.com/path".to_string(),
        scheme_end: 4,
        username_end: 0,
        host_start: 7,
        host_end: 27,
        host: HostInternal::Domain,
        port: None,
        path_start: 28,
        query_start: None,
        fragment_start: None,
    };
    let _ = url.host();
}

#[test]
fn test_host_with_numeric_domain() {
    let mut url = Url {
        serialization: "https://123.456.789.000/path".to_string(),
        scheme_end: 5,
        username_end: 0,
        host_start: 8,
        host_end: 20,
        host: HostInternal::Domain,
        port: None,
        path_start: 21,
        query_start: None,
        fragment_start: None,
    };
    let _ = url.host();
}

