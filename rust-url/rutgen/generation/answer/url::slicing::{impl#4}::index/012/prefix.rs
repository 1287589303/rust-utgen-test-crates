// Answer 0

#[test]
fn test_index_after_path_with_fragment() {
    let url = Url {
        serialization: String::from("http://username:password@host:8080/path#fragment"),
        scheme_end: 4,
        username_end: 13,
        host_start: 16,
        host_end: 20,
        host: HostInternal::Domain,
        port: Some(8080),
        path_start: 21,
        query_start: None,
        fragment_start: Some(29),
    };

    let position = Position::AfterPath;
    let result = url.index(position);
}

#[test]
fn test_index_after_path_with_fragment_at_end() {
    let url = Url {
        serialization: String::from("http://username@host/path#"),
        scheme_end: 4,
        username_end: 15,
        host_start: 16,
        host_end: 20,
        host: HostInternal::Domain,
        port: None,
        path_start: 21,
        query_start: None,
        fragment_start: Some(22),
    };

    let position = Position::AfterPath;
    let result = url.index(position);
}

#[test]
fn test_index_after_path_with_empty_fragment() {
    let url = Url {
        serialization: String::from("http://host/path#"),
        scheme_end: 4,
        username_end: 4,
        host_start: 7,
        host_end: 11,
        host: HostInternal::Domain,
        port: None,
        path_start: 12,
        query_start: None,
        fragment_start: Some(12),
    };

    let position = Position::AfterPath;
    let result = url.index(position);
}

