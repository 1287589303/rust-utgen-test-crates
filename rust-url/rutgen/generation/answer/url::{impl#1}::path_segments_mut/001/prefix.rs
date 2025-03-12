// Answer 0

#[test]
fn test_path_segments_mut_with_no_path() {
    let mut url = Url {
        serialization: String::from("http://example.com"),
        scheme_end: 4,
        username_end: 0,
        host_start: 7,
        host_end: 17,
        host: HostInternal::Domain,
        port: None,
        path_start: 17,
        query_start: None,
        fragment_start: None,
    };
    let result = url.path_segments_mut();
}

#[test]
fn test_path_segments_mut_with_malformed_path() {
    let mut url = Url {
        serialization: String::from("ftp://user@host"),
        scheme_end: 6,
        username_end: 10,
        host_start: 11,
        host_end: 15,
        host: HostInternal::Domain,
        port: None,
        path_start: 15,
        query_start: None,
        fragment_start: None,
    };
    let result = url.path_segments_mut();
}

