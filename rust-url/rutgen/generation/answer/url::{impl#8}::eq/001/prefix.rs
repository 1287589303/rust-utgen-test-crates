// Answer 0

#[test]
fn test_url_eq_same_strings() {
    let url1 = Url {
        serialization: String::from("http://example.com"),
        scheme_end: 4,
        username_end: 0,
        host_start: 7,
        host_end: 11,
        host: HostInternal::Domain,
        port: None,
        path_start: 0,
        query_start: None,
        fragment_start: None,
    };
    let url2 = Url {
        serialization: String::from("http://example.com"),
        scheme_end: 4,
        username_end: 0,
        host_start: 7,
        host_end: 11,
        host: HostInternal::Domain,
        port: None,
        path_start: 0,
        query_start: None,
        fragment_start: None,
    };

    let _ = url1.eq(&url2);
}

#[test]
fn test_url_eq_different_strings() {
    let url1 = Url {
        serialization: String::from("http://example.com"),
        scheme_end: 4,
        username_end: 0,
        host_start: 7,
        host_end: 11,
        host: HostInternal::Domain,
        port: None,
        path_start: 0,
        query_start: None,
        fragment_start: None,
    };
    let url2 = Url {
        serialization: String::from("http://another-example.com"),
        scheme_end: 4,
        username_end: 0,
        host_start: 7,
        host_end: 18,
        host: HostInternal::Domain,
        port: None,
        path_start: 0,
        query_start: None,
        fragment_start: None,
    };

    let _ = url1.eq(&url2);
}

#[test]
fn test_url_eq_empty_strings() {
    let url1 = Url {
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
    let url2 = Url {
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

    let _ = url1.eq(&url2);
}

#[test]
fn test_url_eq_one_empty_string() {
    let url1 = Url {
        serialization: String::from("http://example.com"),
        scheme_end: 4,
        username_end: 0,
        host_start: 7,
        host_end: 11,
        host: HostInternal::Domain,
        port: None,
        path_start: 0,
        query_start: None,
        fragment_start: None,
    };
    let url2 = Url {
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

    let _ = url1.eq(&url2);
}

#[test]
fn test_url_eq_different_lengths() {
    let url1 = Url {
        serialization: String::from("http://example.com"),
        scheme_end: 4,
        username_end: 0,
        host_start: 7,
        host_end: 11,
        host: HostInternal::Domain,
        port: None,
        path_start: 0,
        query_start: None,
        fragment_start: None,
    };
    let url2 = Url {
        serialization: String::from("http://ex.com"),
        scheme_end: 4,
        username_end: 0,
        host_start: 7,
        host_end: 10,
        host: HostInternal::Domain,
        port: None,
        path_start: 0,
        query_start: None,
        fragment_start: None,
    };

    let _ = url1.eq(&url2);
}

