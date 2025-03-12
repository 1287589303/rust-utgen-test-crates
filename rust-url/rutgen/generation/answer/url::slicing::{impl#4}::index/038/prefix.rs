// Answer 0

#[test]
fn test_index_before_username_no_authority() {
    let url = Url {
        serialization: "http://example.com".to_string(),
        scheme_end: 4,
        username_end: 4,
        host_start: 4,
        host_end: 11,
        host: HostInternal::Domain,
        port: None,
        path_start: 11,
        query_start: None,
        fragment_start: None,
    };

    let position = Position::BeforeUsername;
    let _result = url.index(position);
}

#[test]
fn test_index_after_scheme_no_authority() {
    let url = Url {
        serialization: "http://example.com".to_string(),
        scheme_end: 4,
        username_end: 4,
        host_start: 4,
        host_end: 11,
        host: HostInternal::Domain,
        port: None,
        path_start: 11,
        query_start: None,
        fragment_start: None,
    };

    let position = Position::AfterScheme;
    let _result = url.index(position);
}

#[test]
fn test_index_after_username_no_authority() {
    let url = Url {
        serialization: "http://example.com".to_string(),
        scheme_end: 4,
        username_end: 4,
        host_start: 4,
        host_end: 11,
        host: HostInternal::Domain,
        port: None,
        path_start: 11,
        query_start: None,
        fragment_start: None,
    };

    let position = Position::AfterUsername;
    let _result = url.index(position);
}

