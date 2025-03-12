// Answer 0

#[test]
fn test_index_position_after_password_with_authority() {
    let url = Url {
        serialization: "http://username@host:8080/path".to_string(),
        scheme_end: 4, // length of "http"
        username_end: 10, // index just after "username"
        host_start: 11, // index of "host"
        host_end: 15, // index just after "host"
        host: HostInternal::Domain,
        port: Some(8080),
        path_start: 16, // index of "/path"
        query_start: None,
        fragment_start: None,
    };
    
    let position = Position::AfterPassword;
    
    let result = url.index(position);
}

#[test]
fn test_index_position_after_password_with_authority_non_colon() {
    let url = Url {
        serialization: "https://user@host:3000".to_string(),
        scheme_end: 5, // length of "https"
        username_end: 9, // index just after "user"
        host_start: 10, // index of "host"
        host_end: 14, // index just after "host"
        host: HostInternal::Domain,
        port: Some(3000),
        path_start: 15, // no path in this case
        query_start: None,
        fragment_start: None,
    };
    
    let position = Position::AfterPassword;
    
    let result = url.index(position);
}

#[test]
fn test_index_position_after_password_with_authority_no_colon() {
    let url = Url {
        serialization: "ftp://user@host/path".to_string(),
        scheme_end: 6, // length of "ftp://"
        username_end: 10, // index just after "user"
        host_start: 11, // index of "host"
        host_end: 15, // index just after "host"
        host: HostInternal::Domain,
        port: None,
        path_start: 16, // index of "/path"
        query_start: None,
        fragment_start: None,
    };

    let position = Position::AfterPassword;

    let result = url.index(position);
}

