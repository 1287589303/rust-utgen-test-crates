// Answer 0

#[test]
fn test_index_after_fragment_empty_url() {
    let url = Url {
        serialization: String::from(""),
        scheme_end: 0,
        username_end: 0,
        host_start: 0,
        host_end: 0,
        host: HostInternal::None,
        port: None,
        path_start: 0,
        query_start: None,
        fragment_start: None,
    };
    let _ = url.index(Position::AfterFragment);
}

#[test]
fn test_index_after_fragment_complete_url() {
    let url = Url {
        serialization: String::from("http://user:pass@host.com:80/path?query#fragment"),
        scheme_end: 4,
        username_end: 8,
        host_start: 12,
        host_end: 21,
        host: HostInternal::Domain,
        port: Some(80),
        path_start: 25,
        query_start: Some(31),
        fragment_start: Some(37),
    };
    let _ = url.index(Position::AfterFragment);
}

#[test]
fn test_index_after_fragment_url_with_path_only() {
    let url = Url {
        serialization: String::from("http://host.com/path"),
        scheme_end: 4,
        username_end: 4,
        host_start: 7,
        host_end: 16,
        host: HostInternal::Domain,
        port: None,
        path_start: 17,
        query_start: None,
        fragment_start: None,
    };
    let _ = url.index(Position::AfterFragment);
}

#[test]
fn test_index_after_fragment_url_with_query() {
    let url = Url {
        serialization: String::from("http://host.com/path?query"),
        scheme_end: 4,
        username_end: 4,
        host_start: 7,
        host_end: 16,
        host: HostInternal::Domain,
        port: None,
        path_start: 17,
        query_start: Some(23),
        fragment_start: None,
    };
    let _ = url.index(Position::AfterFragment);
}

#[test]
fn test_index_after_fragment_url_with_fragment() {
    let url = Url {
        serialization: String::from("http://host.com/path?query#fragment"),
        scheme_end: 4,
        username_end: 4,
        host_start: 7,
        host_end: 16,
        host: HostInternal::Domain,
        port: None,
        path_start: 17,
        query_start: Some(23),
        fragment_start: Some(31),
    };
    let _ = url.index(Position::AfterFragment);
}

