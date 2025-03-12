// Answer 0

#[test]
fn test_index_before_scheme_to_after_scheme() {
    let url = Url {
        serialization: String::from("http://example.com"),
        scheme_end: 4,
        username_end: 4,
        host_start: 7,
        host_end: 13,
        host: HostInternal::Domain,
        port: None,
        path_start: 13,
        query_start: None,
        fragment_start: None,
    };
    let range = Range {
        start: Position::BeforeScheme,
        end: Position::AfterScheme,
    };
    let _result = url.index(range);
}

#[test]
fn test_index_after_scheme_to_before_username() {
    let url = Url {
        serialization: String::from("http://user:pass@example.com"),
        scheme_end: 4,
        username_end: 8,
        host_start: 10,
        host_end: 18,
        host: HostInternal::Domain,
        port: None,
        path_start: 18,
        query_start: None,
        fragment_start: None,
    };
    let range = Range {
        start: Position::AfterScheme,
        end: Position::BeforeUsername,
    };
    let _result = url.index(range);
}

#[test]
fn test_index_before_username_to_after_username() {
    let url = Url {
        serialization: String::from("ftp://user:pass@host:21/path"),
        scheme_end: 5,
        username_end: 9,
        host_start: 13,
        host_end: 17,
        host: HostInternal::Domain,
        port: Some(21),
        path_start: 21,
        query_start: None,
        fragment_start: None,
    };
    let range = Range {
        start: Position::BeforeUsername,
        end: Position::AfterUsername,
    };
    let _result = url.index(range);
}

#[test]
fn test_index_before_host_to_after_host() {
    let url = Url {
        serialization: String::from("http://example.com/path?query#fragment"),
        scheme_end: 4,
        username_end: 4,
        host_start: 7,
        host_end: 18,
        host: HostInternal::Domain,
        port: None,
        path_start: 22,
        query_start: Some(23),
        fragment_start: Some(30),
    };
    let range = Range {
        start: Position::BeforeHost,
        end: Position::AfterHost,
    };
    let _result = url.index(range);
}

#[test]
fn test_index_before_path_to_after_path() {
    let url = Url {
        serialization: String::from("http://example.com:80/path/to/resource"),
        scheme_end: 4,
        username_end: 4,
        host_start: 7,
        host_end: 18,
        host: HostInternal::Domain,
        port: Some(80),
        path_start: 19,
        query_start: None,
        fragment_start: None,
    };
    let range = Range {
        start: Position::BeforePath,
        end: Position::AfterPath,
    };
    let _result = url.index(range);
}

#[test]
fn test_index_before_query_to_after_query() {
    let url = Url {
        serialization: String::from("http://example.com/path?query=1#fragment"),
        scheme_end: 4,
        username_end: 4,
        host_start: 7,
        host_end: 18,
        host: HostInternal::Domain,
        port: None,
        path_start: 22,
        query_start: Some(28),
        fragment_start: Some(36),
    };
    let range = Range {
        start: Position::BeforeQuery,
        end: Position::AfterQuery,
    };
    let _result = url.index(range);
}

#[test]
fn test_index_before_fragment_to_after_fragment() {
    let url = Url {
        serialization: String::from("http://example.com/path#fragment"),
        scheme_end: 4,
        username_end: 4,
        host_start: 7,
        host_end: 18,
        host: HostInternal::Domain,
        port: None,
        path_start: 22,
        query_start: None,
        fragment_start: Some(23),
    };
    let range = Range {
        start: Position::BeforeFragment,
        end: Position::AfterFragment,
    };
    let _result = url.index(range);
}

