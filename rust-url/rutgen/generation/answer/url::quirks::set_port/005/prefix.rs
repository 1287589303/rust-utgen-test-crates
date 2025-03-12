// Answer 0

#[test]
fn test_set_port_no_host_empty_string() {
    let mut url = Url {
        serialization: String::from("http:///"),
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
    let new_port = "";
    let result = set_port(&mut url, new_port);
}

#[test]
fn test_set_port_no_host_non_empty_string() {
    let mut url = Url {
        serialization: String::from("http:///"),
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
    let new_port = "8080";
    let result = set_port(&mut url, new_port);
}

