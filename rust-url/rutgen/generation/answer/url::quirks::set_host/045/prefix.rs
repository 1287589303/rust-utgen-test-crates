// Answer 0

#[test]
fn test_set_host_return_err_due_to_cannot_be_a_base() {
    let mut url = Url {
        serialization: "http://username:password@host.com/path".to_string(),
        scheme_end: 4,
        username_end: 20,
        host_start: 21,
        host_end: 30,
        host: Host::Domain("host.com".to_string()),
        port: None,
        path_start: 31,
        query_start: None,
        fragment_start: None,
    };

    let new_host = "";
    let result = set_host(&mut url, new_host);
}

#[test]
fn test_set_host_return_err_due_to_host_parsing_fail() {
    let mut url = Url {
        serialization: "http://username@host.com/path".to_string(),
        scheme_end: 4,
        username_end: 10,
        host_start: 11,
        host_end: 20,
        host: Host::Domain("host.com".to_string()),
        port: None,
        path_start: 21,
        query_start: None,
        fragment_start: None,
    };

    let new_host = "invalid_host_format";
    let result = set_host(&mut url, new_host);
}

#[test]
fn test_set_host_return_err_due_to_remaining_split_prefix() {
    let mut url = Url {
        serialization: "http://username@host.com/path".to_string(),
        scheme_end: 4,
        username_end: 10,
        host_start: 11,
        host_end: 20,
        host: Host::Domain("host.com".to_string()),
        port: None,
        path_start: 21,
        query_start: None,
        fragment_start: None,
    };

    let new_host = "host:80";
    let result = set_host(&mut url, new_host);
}

#[test]
fn test_set_host_return_err_due_to_remaining_empty() {
    let mut url = Url {
        serialization: "http://username@host.com/path".to_string(),
        scheme_end: 4,
        username_end: 10,
        host_start: 11,
        host_end: 20,
        host: Host::Domain("host.com".to_string()),
        port: Some(80),
        path_start: 21,
        query_start: None,
        fragment_start: None,
    };

    let new_host = "host.com:";
    let result = set_host(&mut url, new_host);
}

#[test]
fn test_set_host_return_err_due_to_host_is_empty() {
    let mut url = Url {
        serialization: "http://username@host.com/path".to_string(),
        scheme_end: 4,
        username_end: 10,
        host_start: 11,
        host_end: 20,
        host: Host::Domain("host.com".to_string()),
        port: Some(80),
        path_start: 21,
        query_start: None,
        fragment_start: None,
    };

    let new_host = "";
    let result = set_host(&mut url, new_host);
}

