// Answer 0

#[test]
fn test_index_full_range() {
    let url = Url {
        serialization: String::from("http://example.com"),
        scheme_end: 4,
        username_end: 0,
        host_start: 0,
        host_end: 15,
        host: HostInternal::Domain,
        port: None,
        path_start: 0,
        query_start: None,
        fragment_start: None,
    };
    let _result = url.index(0..url.serialization.len());
}

#[test]
fn test_index_empty_range() {
    let url = Url {
        serialization: String::from("http://example.com"),
        scheme_end: 4,
        username_end: 0,
        host_start: 0,
        host_end: 15,
        host: HostInternal::Domain,
        port: None,
        path_start: 0,
        query_start: None,
        fragment_start: None,
    };
    let _result = url.index(..0);
}

#[test]
fn test_index_partial_range() {
    let url = Url {
        serialization: String::from("http://example.com"),
        scheme_end: 4,
        username_end: 0,
        host_start: 0,
        host_end: 15,
        host: HostInternal::Domain,
        port: None,
        path_start: 0,
        query_start: None,
        fragment_start: None,
    };
    let _result = url.index(0..5);
}

#[test]
fn test_index_full_length_range() {
    let url = Url {
        serialization: String::from("http://example.com"),
        scheme_end: 4,
        username_end: 0,
        host_start: 0,
        host_end: 15,
        host: HostInternal::Domain,
        port: None,
        path_start: 0,
        query_start: None,
        fragment_start: None,
    };
    let _result = url.index(0..url.serialization.len());
}

