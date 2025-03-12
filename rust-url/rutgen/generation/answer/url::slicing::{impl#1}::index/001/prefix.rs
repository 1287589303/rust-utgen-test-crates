// Answer 0

#[test]
fn test_index_before_scheme() {
    let url = Url {
        serialization: String::from("http://example.com"),
        scheme_end: 4,
        username_end: 0,
        host_start: 0,
        host_end: 0,
        host: HostInternal::Domain,
        port: None,
        path_start: 0,
        query_start: None,
        fragment_start: None,
    };
    let range = RangeFrom { start: Position::BeforeScheme };
    let _result = url.index(range);
}

#[test]
fn test_index_after_scheme() {
    let url = Url {
        serialization: String::from("https://user:pass@example.com:8080/path?query#fragment"),
        scheme_end: 5,
        username_end: 9,
        host_start: 10,
        host_end: 21,
        host: HostInternal::Domain,
        port: Some(8080),
        path_start: 22,
        query_start: Some(28),
        fragment_start: Some(34),
    };
    let range = RangeFrom { start: Position::AfterScheme };
    let _result = url.index(range);
}

#[test]
fn test_index_before_username() {
    let url = Url {
        serialization: String::from("ftp://user@host/path"),
        scheme_end: 3,
        username_end: 7,
        host_start: 8,
        host_end: 12,
        host: HostInternal::Domain,
        port: None,
        path_start: 13,
        query_start: None,
        fragment_start: None,
    };
    let range = RangeFrom { start: Position::BeforeUsername };
    let _result = url.index(range);
}

#[test]
fn test_index_after_username() {
    let url = Url {
        serialization: String::from("ftp://user@host/path"),
        scheme_end: 3,
        username_end: 7,
        host_start: 8,
        host_end: 12,
        host: HostInternal::Domain,
        port: None,
        path_start: 13,
        query_start: None,
        fragment_start: None,
    };
    let range = RangeFrom { start: Position::AfterUsername };
    let _result = url.index(range);
}

#[test]
fn test_index_before_host() {
    let url = Url {
        serialization: String::from("http://example.com"),
        scheme_end: 4,
        username_end: 0,
        host_start: 0,
        host_end: 11,
        host: HostInternal::Domain,
        port: None,
        path_start: 0,
        query_start: None,
        fragment_start: None,
    };
    let range = RangeFrom { start: Position::BeforeHost };
    let _result = url.index(range);
}

#[test]
fn test_index_after_host() {
    let url = Url {
        serialization: String::from("http://example.com/path"),
        scheme_end: 4,
        username_end: 0,
        host_start: 0,
        host_end: 11,
        host: HostInternal::Domain,
        port: None,
        path_start: 12,
        query_start: None,
        fragment_start: None,
    };
    let range = RangeFrom { start: Position::AfterHost };
    let _result = url.index(range);
}

#[test]
fn test_index_before_path() {
    let url = Url {
        serialization: String::from("http://example.com/path"),
        scheme_end: 4,
        username_end: 0,
        host_start: 0,
        host_end: 11,
        host: HostInternal::Domain,
        port: None,
        path_start: 12,
        query_start: None,
        fragment_start: None,
    };
    let range = RangeFrom { start: Position::BeforePath };
    let _result = url.index(range);
}

#[test]
fn test_index_after_path() {
    let url = Url {
        serialization: String::from("http://example.com/path?query#fragment"),
        scheme_end: 4,
        username_end: 0,
        host_start: 0,
        host_end: 11,
        host: HostInternal::Domain,
        port: None,
        path_start: 12,
        query_start: Some(18),
        fragment_start: Some(26),
    };
    let range = RangeFrom { start: Position::AfterPath };
    let _result = url.index(range);
}

#[test]
fn test_index_before_query() {
    let url = Url {
        serialization: String::from("http://example.com/path?query#fragment"),
        scheme_end: 4,
        username_end: 0,
        host_start: 0,
        host_end: 11,
        host: HostInternal::Domain,
        port: None,
        path_start: 12,
        query_start: Some(18),
        fragment_start: Some(26),
    };
    let range = RangeFrom { start: Position::BeforeQuery };
    let _result = url.index(range);
}

#[test]
fn test_index_after_query() {
    let url = Url {
        serialization: String::from("http://example.com/path?query#fragment"),
        scheme_end: 4,
        username_end: 0,
        host_start: 0,
        host_end: 11,
        host: HostInternal::Domain,
        port: None,
        path_start: 12,
        query_start: Some(18),
        fragment_start: Some(26),
    };
    let range = RangeFrom { start: Position::AfterQuery };
    let _result = url.index(range);
}

#[test]
fn test_index_before_fragment() {
    let url = Url {
        serialization: String::from("http://example.com/path?query#fragment"),
        scheme_end: 4,
        username_end: 0,
        host_start: 0,
        host_end: 11,
        host: HostInternal::Domain,
        port: None,
        path_start: 12,
        query_start: Some(18),
        fragment_start: Some(26),
    };
    let range = RangeFrom { start: Position::BeforeFragment };
    let _result = url.index(range);
}

#[test]
fn test_index_after_fragment() {
    let url = Url {
        serialization: String::from("http://example.com/path?query#fragment"),
        scheme_end: 4,
        username_end: 0,
        host_start: 0,
        host_end: 11,
        host: HostInternal::Domain,
        port: None,
        path_start: 12,
        query_start: Some(18),
        fragment_start: Some(26),
    };
    let range = RangeFrom { start: Position::AfterFragment };
    let _result = url.index(range);
}

