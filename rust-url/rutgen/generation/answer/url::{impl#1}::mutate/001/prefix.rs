// Answer 0

#[test]
fn test_mutate_non_empty_serialization() {
    let mut url = Url {
        serialization: String::from("http://example.com"),
        scheme_end: 4,
        username_end: 0,
        host_start: 0,
        host_end: 15,
        host: HostInternal::Domain,
        port: Some(80),
        path_start: 0,
        query_start: None,
        fragment_start: None,
    };
    url.mutate(|parser| {
        parser.serialization = String::from("https://example.com");
    });
}

#[test]
fn test_mutate_with_complex_function() {
    let mut url = Url {
        serialization: String::from("http://example.com"),
        scheme_end: 4,
        username_end: 0,
        host_start: 0,
        host_end: 15,
        host: HostInternal::Domain,
        port: Some(80),
        path_start: 0,
        query_start: None,
        fragment_start: None,
    };
    url.mutate(|parser| {
        parser.serialization.push_str("/path");
    });
}

#[test]
fn test_mutate_empty_path_modification() {
    let mut url = Url {
        serialization: String::from("http://example.com"),
        scheme_end: 4,
        username_end: 0,
        host_start: 0,
        host_end: 15,
        host: HostInternal::Domain,
        port: Some(80),
        path_start: 0,
        query_start: None,
        fragment_start: None,
    };
    url.mutate(|parser| {
        parser.serialization.clear();
        parser.serialization.push_str("http://newurl.com");
    });
}

#[test]
fn test_mutate_with_no_change_function() {
    let mut url = Url {
        serialization: String::from("http://example.com"),
        scheme_end: 4,
        username_end: 0,
        host_start: 0,
        host_end: 15,
        host: HostInternal::Domain,
        port: Some(80),
        path_start: 0,
        query_start: None,
        fragment_start: None,
    };
    url.mutate(|_| {});
}

