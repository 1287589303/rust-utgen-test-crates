// Answer 0

#[test]
fn test_index_after_password_with_authority() {
    let url = Url {
        serialization: String::from("http://username@host.com:8080/path?query#fragment"),
        scheme_end: 4, // Position after "http"
        username_end: 13, // Position after "username"
        host_start: 14, // Position before "host.com"
        host_end: 25, // Position after "host.com"
        host: HostInternal::Domain,
        port: Some(8080),
        path_start: 26, // Position before "/path"
        query_start: Some(31), // Position before "?query"
        fragment_start: Some(37), // Position before "#fragment"
    };
    
    let position = Position::AfterPassword;
    url.index(position);
}

