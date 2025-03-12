// Answer 0

#[test]
fn test_set_hostname_empty_hostname_special_not_file() {
    let mut url = Url {
        serialization: "http://user:pass@localhost:8080".to_string(),
        scheme_end: 4,
        username_end: 8,
        host_start: 12,
        host_end: 20,
        host: Host::Domain("localhost".to_string()),
        port: Some(8080),
        path_start: 20,
        query_start: None,
        fragment_start: None,
    };
    let new_hostname = "";
    let result = set_hostname(&mut url, new_hostname);
}

#[test]
fn test_set_hostname_empty_hostname_special_not_file_with_username() {
    let mut url = Url {
        serialization: "http://user@localhost:8080".to_string(),
        scheme_end: 4,
        username_end: 4,
        host_start: 10,
        host_end: 18,
        host: Host::Domain("localhost".to_string()),
        port: Some(8080),
        path_start: 18,
        query_start: None,
        fragment_start: None,
    };
    let new_hostname = "";
    let result = set_hostname(&mut url, new_hostname);
}

#[test]
fn test_set_hostname_empty_hostname_special_not_file_with_password() {
    let mut url = Url {
        serialization: "http://:pass@localhost:8080".to_string(),
        scheme_end: 4,
        username_end: 0,
        host_start: 10,
        host_end: 18,
        host: Host::Domain("localhost".to_string()),
        port: Some(8080),
        path_start: 18,
        query_start: None,
        fragment_start: None,
    };
    let new_hostname = "";
    let result = set_hostname(&mut url, new_hostname);
}

