// Answer 0

#[test]
fn test_set_password_empty_username() {
    let mut url = Url {
        serialization: String::from("http://user@domain.com/path"),
        scheme_end: 4,
        username_end: 4,
        host_start: 8,
        host_end: 18,
        host: HostInternal::Domain,
        port: None,
        path_start: 22,
        query_start: Some(40),
        fragment_start: Some(50),
    };
    
    let result = url.set_password(None);
}

#[test]
fn test_set_password_remove_existing() {
    let mut url = Url {
        serialization: String::from("http://user:pass@domain.com/path?query#fragment"),
        scheme_end: 4,
        username_end: 4,
        host_start: 16,
        host_end: 26,
        host: HostInternal::Domain,
        port: None,
        path_start: 30,
        query_start: Some(36),
        fragment_start: Some(46),
    };
    
    let result = url.set_password(None);
}

#[test]
fn test_set_password_remove_pass_when_no_user() {
    let mut url = Url {
        serialization: String::from("http://:pass@domain.com/path?query#fragment"),
        scheme_end: 4,
        username_end: 5,
        host_start: 9,
        host_end: 19,
        host: HostInternal::Domain,
        port: None,
        path_start: 23,
        query_start: Some(29),
        fragment_start: Some(39),
    };
    
    let result = url.set_password(None);
}

