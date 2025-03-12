// Answer 0

#[test]
fn test_username_with_authority_and_empty_username() {
    let url = Url {
        serialization: String::from("ftp://:secret@example.com"),
        scheme_end: 4,
        username_end: 4,
        host_start: 7,
        host_end: 15,
        host: HostInternal::Domain,
        port: None,
        path_start: 15,
        query_start: None,
        fragment_start: None,
    };
    let _ = url.username();
}

#[test]
fn test_username_with_authority_and_no_username() {
    let url = Url {
        serialization: String::from("ftp://:example.com"),
        scheme_end: 4,
        username_end: 4,
        host_start: 6,
        host_end: 14,
        host: HostInternal::Domain,
        port: None,
        path_start: 14,
        query_start: None,
        fragment_start: None,
    };
    let _ = url.username();
}

#[test]
fn test_username_with_authority_and_no_credentials() {
    let url = Url {
        serialization: String::from("https://example.com"),
        scheme_end: 5,
        username_end: 5,
        host_start: 8,
        host_end: 16,
        host: HostInternal::Domain,
        port: None,
        path_start: 16,
        query_start: None,
        fragment_start: None,
    };
    let _ = url.username();
}

