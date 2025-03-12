// Answer 0

#[test]
fn test_index_after_host_valid_url() {
    let url = Url {
        serialization: String::from("http://username:password@host:80/path?query#fragment"),
        scheme_end: 4, // length of "http"
        username_end: 13, // length of "username:"
        host_start: 15, // start of "host"
        host_end: 19, // length of "host"
        host: HostInternal::Domain,
        port: Some(80),
        path_start: 24, // start of "/path"
        query_start: Some(29), // start of "?query"
        fragment_start: Some(35), // start of "#fragment"
    };

    let position = Position::AfterHost;
    let result = url.index(position);
}

#[test]
fn test_index_after_host_no_port() {
    let url = Url {
        serialization: String::from("https://user@domain.com/path"),
        scheme_end: 5, // length of "https"
        username_end: 9, // length of "user@"
        host_start: 10, // start of "domain.com"
        host_end: 21, // length of "domain.com"
        host: HostInternal::Domain,
        port: None,
        path_start: 25, // start of "/path"
        query_start: None,
        fragment_start: None,
    };

    let position = Position::AfterHost;
    let result = url.index(position);
}

