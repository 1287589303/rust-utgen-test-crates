// Answer 0

#[test]
fn test_set_pathname_with_root_slash() {
    let mut url = Url {
        serialization: String::from("http://example.com"),
        scheme_end: 4,
        username_end: 0,
        host_start: 7,
        host_end: 14,
        host: HostInternal::Domain(String::from("example.com")),
        port: None,
        path_start: 0,
        query_start: None,
        fragment_start: None,
    };
    let new_pathname = "/new/path";
    set_pathname(&mut url, new_pathname);
}

#[test]
fn test_set_pathname_with_simple_path() {
    let mut url = Url {
        serialization: String::from("http://example.com"),
        scheme_end: 4,
        username_end: 0,
        host_start: 7,
        host_end: 14,
        host: HostInternal::Domain(String::from("example.com")),
        port: None,
        path_start: 0,
        query_start: None,
        fragment_start: None,
    };
    let new_pathname = "/another/path";
    set_pathname(&mut url, new_pathname);
}

#[test]
fn test_set_pathname_with_special_scheme() {
    let mut url = Url {
        serialization: String::from("file:///home/user/docs"),
        scheme_end: 4,
        username_end: 0,
        host_start: 0,
        host_end: 0,
        host: HostInternal::None,
        port: None,
        path_start: 0,
        query_start: None,
        fragment_start: None,
    };
    let new_pathname = "/new/file.txt";
    set_pathname(&mut url, new_pathname);
}

#[test]
fn test_set_pathname_with_empty_path_after_slash() {
    let mut url = Url {
        serialization: String::from("http://example.com"),
        scheme_end: 4,
        username_end: 0,
        host_start: 7,
        host_end: 14,
        host: HostInternal::Domain(String::from("example.com")),
        port: None,
        path_start: 0,
        query_start: None,
        fragment_start: None,
    };
    let new_pathname = "/";
    set_pathname(&mut url, new_pathname);
}

