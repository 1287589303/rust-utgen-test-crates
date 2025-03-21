// Answer 0

#[test]
fn test_index_before_scheme() {
    let url = Url {
        serialization: String::from("http://example.com"),
        scheme_end: 4,
        username_end: 4,
        host_start: 7,
        host_end: 18,
        host: HostInternal::Domain,
        port: None,
        path_start: 19,
        query_start: None,
        fragment_start: None,
    };
    let position = Position::BeforeScheme;
    let _ = url.index(position);
}

#[test]
fn test_index_after_scheme() {
    let url = Url {
        serialization: String::from("https://example.com/path"),
        scheme_end: 5,
        username_end: 5,
        host_start: 8,
        host_end: 18,
        host: HostInternal::Domain,
        port: None,
        path_start: 19,
        query_start: None,
        fragment_start: None,
    };
    let position = Position::AfterScheme;
    let _ = url.index(position);
}

#[test]
fn test_index_before_username() {
    let url = Url {
        serialization: String::from("http://user:pass@example.com"),
        scheme_end: 4,
        username_end: 8,
        host_start: 12,
        host_end: 22,
        host: HostInternal::Domain,
        port: None,
        path_start: 23,
        query_start: None,
        fragment_start: None,
    };
    let position = Position::BeforeUsername;
    let _ = url.index(position);
}

#[test]
fn test_index_after_username() {
    let url = Url {
        serialization: String::from("http://user:pass@example.com"),
        scheme_end: 4,
        username_end: 8,
        host_start: 12,
        host_end: 22,
        host: HostInternal::Domain,
        port: None,
        path_start: 23,
        query_start: None,
        fragment_start: None,
    };
    let position = Position::AfterUsername;
    let _ = url.index(position);
}

#[test]
fn test_index_before_password() {
    let url = Url {
        serialization: String::from("http://user:pass@example.com"),
        scheme_end: 4,
        username_end: 8,
        host_start: 12,
        host_end: 22,
        host: HostInternal::Domain,
        port: None,
        path_start: 23,
        query_start: None,
        fragment_start: None,
    };
    let position = Position::BeforePassword;
    let _ = url.index(position);
}

#[test]
fn test_index_after_password() {
    let url = Url {
        serialization: String::from("http://user:pass@example.com"),
        scheme_end: 4,
        username_end: 8,
        host_start: 12,
        host_end: 22,
        host: HostInternal::Domain,
        port: None,
        path_start: 23,
        query_start: None,
        fragment_start: None,
    };
    let position = Position::AfterPassword;
    let _ = url.index(position);
}

#[test]
fn test_index_before_host() {
    let url = Url {
        serialization: String::from("http://example.com"),
        scheme_end: 4,
        username_end: 4,
        host_start: 7,
        host_end: 18,
        host: HostInternal::Domain,
        port: None,
        path_start: 19,
        query_start: None,
        fragment_start: None,
    };
    let position = Position::BeforeHost;
    let _ = url.index(position);
}

#[test]
fn test_index_after_host() {
    let url = Url {
        serialization: String::from("http://example.com:8080/path"),
        scheme_end: 4,
        username_end: 4,
        host_start: 7,
        host_end: 18,
        host: HostInternal::Domain,
        port: Some(8080),
        path_start: 22,
        query_start: None,
        fragment_start: None,
    };
    let position = Position::AfterHost;
    let _ = url.index(position);
}

#[test]
fn test_index_before_port() {
    let url = Url {
        serialization: String::from("http://example.com:8080/path"),
        scheme_end: 4,
        username_end: 4,
        host_start: 7,
        host_end: 18,
        host: HostInternal::Domain,
        port: Some(8080),
        path_start: 22,
        query_start: None,
        fragment_start: None,
    };
    let position = Position::BeforePort;
    let _ = url.index(position);
}

#[test]
fn test_index_after_port() {
    let url = Url {
        serialization: String::from("http://example.com:8080/path"),
        scheme_end: 4,
        username_end: 4,
        host_start: 7,
        host_end: 18,
        host: HostInternal::Domain,
        port: Some(8080),
        path_start: 22,
        query_start: None,
        fragment_start: None,
    };
    let position = Position::AfterPort;
    let _ = url.index(position);
}

#[test]
fn test_index_before_path() {
    let url = Url {
        serialization: String::from("http://example.com/path"),
        scheme_end: 4,
        username_end: 4,
        host_start: 7,
        host_end: 18,
        host: HostInternal::Domain,
        port: None,
        path_start: 19,
        query_start: None,
        fragment_start: None,
    };
    let position = Position::BeforePath;
    let _ = url.index(position);
}

#[test]
fn test_index_after_path() {
    let url = Url {
        serialization: String::from("http://example.com/path?query=1"),
        scheme_end: 4,
        username_end: 4,
        host_start: 7,
        host_end: 18,
        host: HostInternal::Domain,
        port: None,
        path_start: 19,
        query_start: Some(24),
        fragment_start: None,
    };
    let position = Position::AfterPath;
    let _ = url.index(position);
}

#[test]
fn test_index_before_query() {
    let url = Url {
        serialization: String::from("http://example.com/path?query=1"),
        scheme_end: 4,
        username_end: 4,
        host_start: 7,
        host_end: 18,
        host: HostInternal::Domain,
        port: None,
        path_start: 19,
        query_start: Some(24),
        fragment_start: None,
    };
    let position = Position::BeforeQuery;
    let _ = url.index(position);
}

#[test]
fn test_index_after_query() {
    let url = Url {
        serialization: String::from("http://example.com/path?query=1#fragment"),
        scheme_end: 4,
        username_end: 4,
        host_start: 7,
        host_end: 18,
        host: HostInternal::Domain,
        port: None,
        path_start: 19,
        query_start: Some(24),
        fragment_start: Some(34),
    };
    let position = Position::AfterQuery;
    let _ = url.index(position);
}

#[test]
fn test_index_before_fragment() {
    let url = Url {
        serialization: String::from("http://example.com/path#fragment"),
        scheme_end: 4,
        username_end: 4,
        host_start: 7,
        host_end: 18,
        host: HostInternal::Domain,
        port: None,
        path_start: 19,
        query_start: None,
        fragment_start: Some(24),
    };
    let position = Position::BeforeFragment;
    let _ = url.index(position);
}

#[test]
fn test_index_after_fragment() {
    let url = Url {
        serialization: String::from("http://example.com/path#fragment"),
        scheme_end: 4,
        username_end: 4,
        host_start: 7,
        host_end: 18,
        host: HostInternal::Domain,
        port: None,
        path_start: 19,
        query_start: None,
        fragment_start: Some(24),
    };
    let position = Position::AfterFragment;
    let _ = url.index(position);
}

