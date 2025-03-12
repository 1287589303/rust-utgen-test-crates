// Answer 0

#[test]
fn test_set_host_file_scheme_with_empty_host() {
    let mut url = Url {
        serialization: String::from("file:///"),
        scheme_end: 4,
        username_end: 4,
        host_start: 4,
        host_end: 4,
        host: Host::Domain(String::from("")),
        port: None,
        path_start: 5,
        query_start: None,
        fragment_start: None,
    };
    let new_host = "";
    let result = set_host(&mut url, new_host);
}

#[test]
fn test_set_host_file_scheme_with_empty_host_no_username_or_port() {
    let mut url = Url {
        serialization: String::from("file:///path/to/file"),
        scheme_end: 4,
        username_end: 4,
        host_start: 4,
        host_end: 19,
        host: Host::Domain(String::from("example.com")),
        port: None,
        path_start: 19,
        query_start: None,
        fragment_start: None,
    };
    let new_host = "";
    let result = set_host(&mut url, new_host);
}

