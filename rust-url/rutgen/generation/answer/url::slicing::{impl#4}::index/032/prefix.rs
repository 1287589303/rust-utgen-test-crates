// Answer 0

#[test]
fn test_index_before_password_no_authority_username_end_equals_host_start() {
    let url = Url {
        serialization: String::from("http://user@host"),
        scheme_end: 4, // "http".len()
        username_end: 8, // "user".len() + scheme_end + "://".len()
        host_start: 8,
        host_end: 12, // "host".len() + host_start
        host: HostInternal::Domain,
        port: None,
        path_start: 0,
        query_start: None,
        fragment_start: None,
    };

    let position = Position::BeforePassword;
    let _ = url.index(position);
}

#[test]
fn test_index_before_username_no_authority() {
    let url = Url {
        serialization: String::from("http://user@host"),
        scheme_end: 4,
        username_end: 8,
        host_start: 8,
        host_end: 12,
        host: HostInternal::Domain,
        port: None,
        path_start: 0,
        query_start: None,
        fragment_start: None,
    };

    let position = Position::BeforeUsername;
    let _ = url.index(position);
}

#[test]
fn test_index_after_username_no_authority() {
    let url = Url {
        serialization: String::from("http://user@host"),
        scheme_end: 4,
        username_end: 8,
        host_start: 8,
        host_end: 12,
        host: HostInternal::Domain,
        port: None,
        path_start: 0,
        query_start: None,
        fragment_start: None,
    };

    let position = Position::AfterUsername;
    let _ = url.index(position);
}

#[test]
fn test_index_after_password_no_authority() {
    let url = Url {
        serialization: String::from("http://user@host"),
        scheme_end: 4,
        username_end: 8,
        host_start: 8,
        host_end: 12,
        host: HostInternal::Domain,
        port: None,
        path_start: 0,
        query_start: None,
        fragment_start: None,
    };

    let position = Position::AfterPassword;
    let _ = url.index(position);
}

