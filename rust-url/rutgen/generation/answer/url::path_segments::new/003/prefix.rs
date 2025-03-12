// Answer 0

#[test]
fn test_new_with_non_special_scheme_and_empty_after_path() {
    let mut url = Url {
        serialization: String::from("http://example.com"),
        scheme_end: 4,
        username_end: 0,
        host_start: 7,
        host_end: 14,
        host: HostInternal {},
        port: None,
        path_start: 14,
        query_start: None,
        fragment_start: None,
    };
    let result = new(&mut url);
}

#[test]
fn test_new_with_non_special_scheme_and_valid_path_start() {
    let mut url = Url {
        serialization: String::from("http://example.com/path"),
        scheme_end: 4,
        username_end: 0,
        host_start: 7,
        host_end: 14,
        host: HostInternal {},
        port: None,
        path_start: 14,
        query_start: None,
        fragment_start: None,
    };
    let result = new(&mut url);
}

#[test]
fn test_new_with_non_special_scheme_length_equals_path_start() {
    let mut url = Url {
        serialization: String::from("ftp://example.com"),
        scheme_end: 4,
        username_end: 0,
        host_start: 7,
        host_end: 14,
        host: HostInternal {},
        port: None,
        path_start: 14,
        query_start: None,
        fragment_start: None,
    };
    let result = new(&mut url);
}

#[test]
fn test_new_with_non_special_scheme_and_path_only() {
    let mut url = Url {
        serialization: String::from("http://example.com/path/to/resource"),
        scheme_end: 4,
        username_end: 0,
        host_start: 7,
        host_end: 14,
        host: HostInternal {},
        port: None,
        path_start: 14,
        query_start: None,
        fragment_start: None,
    };
    let result = new(&mut url);
}

