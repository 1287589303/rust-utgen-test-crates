// Answer 0

#[test]
fn test_set_protocol_with_http() {
    let mut url = Url {
        serialization: String::new(),
        scheme_end: 0,
        username_end: 0,
        host_start: 0,
        host_end: 0,
        host: HostInternal::default(),
        port: None,
        path_start: 0,
        query_start: None,
        fragment_start: None,
    };
    let new_protocol = "http://";
    let _ = set_protocol(&mut url, new_protocol);
}

#[test]
fn test_set_protocol_with_ftp() {
    let mut url = Url {
        serialization: String::new(),
        scheme_end: 0,
        username_end: 0,
        host_start: 0,
        host_end: 0,
        host: HostInternal::default(),
        port: None,
        path_start: 0,
        query_start: None,
        fragment_start: None,
    };
    let new_protocol = "ftp://";
    let _ = set_protocol(&mut url, new_protocol);
}

#[test]
fn test_set_protocol_with_mailto() {
    let mut url = Url {
        serialization: String::new(),
        scheme_end: 0,
        username_end: 0,
        host_start: 0,
        host_end: 0,
        host: HostInternal::default(),
        port: None,
        path_start: 0,
        query_start: None,
        fragment_start: None,
    };
    let new_protocol = "mailto:example@example.com";
    let _ = set_protocol(&mut url, new_protocol);
}

#[test]
fn test_set_protocol_with_custom_scheme() {
    let mut url = Url {
        serialization: String::new(),
        scheme_end: 0,
        username_end: 0,
        host_start: 0,
        host_end: 0,
        host: HostInternal::default(),
        port: None,
        path_start: 0,
        query_start: None,
        fragment_start: None,
    };
    let new_protocol = "custom-scheme:action";
    let _ = set_protocol(&mut url, new_protocol);
}

