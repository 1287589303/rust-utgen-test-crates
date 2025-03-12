// Answer 0

#[test]
fn test_index_after_password_without_authority() {
    let url = Url {
        serialization: "http://user@host:80/path?query#fragment".to_string(),
        scheme_end: 4,
        username_end: 8,
        host_start: 9,
        host_end: 13,
        host: HostInternal::Domain,
        port: Some(80),
        path_start: 14,
        query_start: Some(19),
        fragment_start: Some(25),
    };

    let position = Position::AfterPassword;
    let result = url.index(position);
}

#[test]
fn test_index_after_password_without_authority_boundary() {
    let url = Url {
        serialization: "http://user@host/path".to_string(),
        scheme_end: 4,
        username_end: 8,
        host_start: 9,
        host_end: 13,
        host: HostInternal::Domain,
        port: None,
        path_start: 14,
        query_start: None,
        fragment_start: None,
    };

    let position = Position::AfterPassword;
    let result = url.index(position);
}

#[test]
fn test_index_after_password_without_authority_empty() {
    let url = Url {
        serialization: "http://@host".to_string(),
        scheme_end: 4,
        username_end: 5,
        host_start: 6,
        host_end: 10,
        host: HostInternal::Domain,
        port: None,
        path_start: 10,
        query_start: None,
        fragment_start: None,
    };

    let position = Position::AfterPassword;
    let result = url.index(position);
}

