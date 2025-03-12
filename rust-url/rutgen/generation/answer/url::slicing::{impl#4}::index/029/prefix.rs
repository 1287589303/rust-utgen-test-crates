// Answer 0

#[test]
fn test_index_position_before_password_with_authority_and_username_not_colon() {
    let url = Url {
        serialization: String::from("http://user@host:80/path?query#fragment"),
        scheme_end: 4, // length of "http"
        username_end: 4, // index after the username "user" (4)
        host_start: 4, // index after "http://"
        host_end: 8, // index after host "host"
        host: HostInternal::Domain,
        port: Some(80),
        path_start: 12, // index after path "/path"
        query_start: Some(17), // index at query "?query"
        fragment_start: Some(24), // index at fragment "#fragment"
    };

    let position = Position::BeforePassword;
    let _ = url.index(position);
}

#[test]
fn test_index_position_before_password_with_authority_and_username_not_colon_second_case() {
    let url = Url {
        serialization: String::from("https://user@domain.com:443/path/to/resource"),
        scheme_end: 5, // length of "https"
        username_end: 10, // index after the username "user"
        host_start: 6, // index after "https://"
        host_end: 17, // index after host "domain.com"
        host: HostInternal::Domain,
        port: Some(443),
        path_start: 21, // index after path "/path/to/resource"
        query_start: None, // no query
        fragment_start: None, // no fragment
    };

    let position = Position::BeforePassword;
    let _ = url.index(position);
}

#[test]
fn test_index_position_before_password_with_no_port() {
    let url = Url {
        serialization: String::from("http://user@simplehost/path"),
        scheme_end: 4, // length of "http"
        username_end: 4, // index after the username "user"
        host_start: 4, // index after "http://"
        host_end: 14, // index after host "simplehost"
        host: HostInternal::Domain,
        port: None, // no port
        path_start: 16, // index after path "/path"
        query_start: None, // no query
        fragment_start: None, // no fragment
    };

    let position = Position::BeforePassword;
    let _ = url.index(position);
}

