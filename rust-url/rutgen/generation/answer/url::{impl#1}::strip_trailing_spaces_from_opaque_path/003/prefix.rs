// Answer 0

#[test]
fn test_strip_trailing_spaces_opaque_path_case_1() {
    let mut url = Url {
        serialization: String::from("http://example.com/path   "),
        scheme_end: 4,
        username_end: 0,
        host_start: 7,
        host_end: 16,
        host: HostInternal::Domain,
        port: None,
        path_start: 22,
        query_start: None,
        fragment_start: None,
    };
    url.strip_trailing_spaces_from_opaque_path();
}

#[test]
fn test_strip_trailing_spaces_opaque_path_case_2() {
    let mut url = Url {
        serialization: String::from("https://example.org/resource   "),
        scheme_end: 5,
        username_end: 0,
        host_start: 8,
        host_end: 17,
        host: HostInternal::Domain,
        port: None,
        path_start: 27,
        query_start: None,
        fragment_start: None,
    };
    url.strip_trailing_spaces_from_opaque_path();
}

#[test]
fn test_strip_trailing_spaces_opaque_path_case_3() {
    let mut url = Url {
        serialization: String::from("ftp://ftp.example.net/dir/file.txt    "),
        scheme_end: 6,
        username_end: 0,
        host_start: 7,
        host_end: 22,
        host: HostInternal::Domain,
        port: None,
        path_start: 28,
        query_start: None,
        fragment_start: None,
    };
    url.strip_trailing_spaces_from_opaque_path();
}

