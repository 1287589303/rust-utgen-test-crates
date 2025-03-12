// Answer 0

#[test]
fn test_set_hostname_valid_input() {
    let mut url = Url {
        serialization: String::from("http://example.com"),
        scheme_end: 4,  // "http"
        username_end: 4,
        host_start: 7,  // after "http://"
        host_end: 18,   // before "/"
        host: Host::Domain(String::from("example.com")),
        port: None,
        path_start: 18,
        query_start: None,
        fragment_start: None,
    };

    let new_hostname = ""; // an empty hostname to meet `h.is_empty()` condition

    let result = set_hostname(&mut url, new_hostname);
}

#[test]
fn test_set_hostname_no_username_or_password() {
    let mut url = Url {
        serialization: String::from("ftp://example.com"),
        scheme_end: 6,  // "ftp"
        username_end: 6,
        host_start: 10, // after "ftp://"
        host_end: 21,   // before "/"
        host: Host::Domain(String::from("example.com")),
        port: None,
        path_start: 21,
        query_start: None,
        fragment_start: None,
    };

    let new_hostname = ""; // an empty hostname

    let result = set_hostname(&mut url, new_hostname);
}

#[test]
fn test_set_hostname_change_to_empty_host() {
    let mut url = Url {
        serialization: String::from("http://example.com"),
        scheme_end: 4,
        username_end: 4,
        host_start: 7,
        host_end: 18,
        host: Host::Domain(String::from("example.com")),
        port: None,
        path_start: 18,
        query_start: None,
        fragment_start: None,
    };

    let new_hostname = ""; // empty host string

    let result = set_hostname(&mut url, new_hostname);
}

#[test]
fn test_set_hostname_valid_domain() {
    let mut url = Url {
        serialization: String::from("http://example.com"),
        scheme_end: 4,
        username_end: 4,
        host_start: 7,
        host_end: 18,
        host: Host::Domain(String::from("example.com")),
        port: None,
        path_start: 18,
        query_start: None,
        fragment_start: None,
    };

    let new_hostname = "validhostname.com"; // valid hostname

    let result = set_hostname(&mut url, new_hostname);
}

