// Answer 0

#[test]
#[should_panic]
fn test_set_host_empty_string() {
    let mut url = Url {
        serialization: String::from("http://username:password@localhost:8080"),
        scheme_end: 4,
        username_end: 10,
        host_start: 11,
        host_end: 19,
        host: Host::Domain(String::from("localhost")),
        port: Some(8080),
        path_start: 20,
        query_start: None,
        fragment_start: None,
    };
    let new_host = "";
    let _: Result<(), ()> = set_host(&mut url, new_host);
}

#[test]
#[should_panic]
fn test_set_host_valid_hostname() {
    let mut url = Url {
        serialization: String::from("http://username:password@localhost:8080"),
        scheme_end: 4,
        username_end: 10,
        host_start: 11,
        host_end: 19,
        host: Host::Domain(String::from("localhost")),
        port: Some(8080),
        path_start: 20,
        query_start: None,
        fragment_start: None,
    };
    let new_host = "hostname";
    let _: Result<(), ()> = set_host(&mut url, new_host);
}

#[test]
#[should_panic]
fn test_set_host_hostname_with_port() {
    let mut url = Url {
        serialization: String::from("http://username:password@localhost:8080"),
        scheme_end: 4,
        username_end: 10,
        host_start: 11,
        host_end: 19,
        host: Host::Domain(String::from("localhost")),
        port: Some(8080),
        path_start: 20,
        query_start: None,
        fragment_start: None,
    };
    let new_host = "hostname:80";
    let _: Result<(), ()> = set_host(&mut url, new_host);
}

#[test]
#[should_panic]
fn test_set_host_hostname_with_empty_port() {
    let mut url = Url {
        serialization: String::from("http://username:password@localhost:8080"),
        scheme_end: 4,
        username_end: 10,
        host_start: 11,
        host_end: 19,
        host: Host::Domain(String::from("localhost")),
        port: Some(8080),
        path_start: 20,
        query_start: None,
        fragment_start: None,
    };
    let new_host = "hostname:";
    let _: Result<(), ()> = set_host(&mut url, new_host);
}

#[test]
#[should_panic]
fn test_set_host_hostname_with_extra_characters() {
    let mut url = Url {
        serialization: String::from("http://username:password@localhost:8080"),
        scheme_end: 4,
        username_end: 10,
        host_start: 11,
        host_end: 19,
        host: Host::Domain(String::from("localhost")),
        port: Some(8080),
        path_start: 20,
        query_start: None,
        fragment_start: None,
    };
    let new_host = "hostname/extra";
    let _: Result<(), ()> = set_host(&mut url, new_host);
}

#[test]
#[should_panic]
fn test_set_host_userinfo_presence() {
    let mut url = Url {
        serialization: String::from("http://username:password@localhost:8080"),
        scheme_end: 4,
        username_end: 10,
        host_start: 11,
        host_end: 19,
        host: Host::Domain(String::from("localhost")),
        port: Some(8080),
        path_start: 20,
        query_start: None,
        fragment_start: None,
    };
    let new_host = "user:pass@hostname";
    let _: Result<(), ()> = set_host(&mut url, new_host);
}

