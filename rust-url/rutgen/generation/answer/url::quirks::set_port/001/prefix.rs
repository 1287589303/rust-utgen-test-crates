// Answer 0

#[test]
fn test_set_port_with_empty_domain_host_and_file_scheme() {
    let mut url = Url {
        serialization: String::from("file://"),
        scheme_end: 4,
        username_end: 0,
        host_start: 7,
        host_end: 7,
        host: Host::Domain(String::new()),
        port: None,
        path_start: 0,
        query_start: None,
        fragment_start: None,
    };
    let new_port = "8080";
    let result = set_port(&mut url, new_port);
}

#[test]
fn test_set_port_with_empty_domain_host_and_file_scheme_string() {
    let mut url = Url {
        serialization: String::from("file://example.com"),
        scheme_end: 4,
        username_end: 0,
        host_start: 7,
        host_end: 7,
        host: Host::Domain(String::new()),
        port: None,
        path_start: 16,
        query_start: None,
        fragment_start: None,
    };
    let new_port = "80";
    let result = set_port(&mut url, new_port);
}

#[test]
fn test_set_port_with_empty_domain_host_and_file_scheme_numeric() {
    let mut url = Url {
        serialization: String::from("file://"),
        scheme_end: 4,
        username_end: 0,
        host_start: 7,
        host_end: 7,
        host: Host::Domain(String::new()),
        port: None,
        path_start: 0,
        query_start: None,
        fragment_start: None,
    };
    let new_port = "3030";
    let result = set_port(&mut url, new_port);
}

