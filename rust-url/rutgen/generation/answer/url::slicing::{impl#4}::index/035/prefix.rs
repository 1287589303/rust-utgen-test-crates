// Answer 0

#[test]
fn test_index_before_username_with_authority() {
    let url = Url {
        serialization: "http://username:password@host:80/path?query#fragment".to_string(),
        scheme_end: 4, // "http" length
        username_end: 12, // "username" length (4 + length of "://")
        host_start: 13, // index after "username:password@"
        host_end: 14,
        host: HostInternal::Domain,
        port: Some(80),
        path_start: 15,
        query_start: Some(19),
        fragment_start: Some(25),
    };
    let position = Position::BeforeUsername;
    let _ = url.index(position);
}

#[test]
fn test_index_after_scheme_with_authority() {
    let url = Url {
        serialization: "https://user:pass@localhost:8080".to_string(),
        scheme_end: 5, // "https" length
        username_end: 9, // "user" length (5 + length of "://")
        host_start: 10, // index after "user:pass@"
        host_end: 12,
        host: HostInternal::Domain,
        port: Some(8080),
        path_start: 13,
        query_start: None,
        fragment_start: None,
    };
    let position = Position::BeforeUsername;
    let _ = url.index(position);
}

#[test]
fn test_index_before_username_boundary_case() {
    let url = Url {
        serialization: "ftp://:password@127.0.0.1".to_string(),
        scheme_end: 4, // "ftp" length
        username_end: 5, // only password, so username_end is right after "://"
        host_start: 9, // index after "127.0.0.1"
        host_end: 10,
        host: HostInternal::Ipv4(Ipv4Addr::new(127, 0, 0, 1)),
        port: None,
        path_start: 11,
        query_start: None,
        fragment_start: None,
    };
    let position = Position::BeforeUsername;
    let _ = url.index(position);
}

