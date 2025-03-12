// Answer 0

#[test]
fn test_byte_at_valid_start() {
    let url = Url {
        serialization: "http://example.com".to_owned(),
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
    let _ = url.byte_at(0);
}

#[test]
fn test_byte_at_valid_end() {
    let url = Url {
        serialization: "http://example.com".to_owned(),
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
    let _ = url.byte_at((url.serialization.len() as u32 - 1));
}

#[should_panic]
#[test]
fn test_byte_at_out_of_bounds() {
    let url = Url {
        serialization: "http://example.com".to_owned(),
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
    let _ = url.byte_at(url.serialization.len() as u32);
}

