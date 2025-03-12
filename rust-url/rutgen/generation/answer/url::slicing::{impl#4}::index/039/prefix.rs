// Answer 0

#[test]
fn test_index_after_scheme() {
    let url = Url {
        serialization: String::from("http://username:password@host:8080/path?query#fragment"),
        scheme_end: 4,
        username_end: 16,
        host_start: 17,
        host_end: 21,
        host: HostInternal::Domain,
        port: Some(8080),
        path_start: 22,
        query_start: Some(28),
        fragment_start: Some(34),
    };
    
    let result = url.index(Position::AfterScheme);
}

#[test]
fn test_index_before_username() {
    let url = Url {
        serialization: String::from("https://user:pass@localhost:3000"),
        scheme_end: 5,
        username_end: 9,
        host_start: 10,
        host_end: 13,
        host: HostInternal::Domain,
        port: Some(3000),
        path_start: 14,
        query_start: None,
        fragment_start: None,
    };

    let result = url.index(Position::BeforeUsername);
}

#[test]
fn test_index_after_username() {
    let url = Url {
        serialization: String::from("ftp://user:pass@host/path"),
        scheme_end: 6,
        username_end: 10,
        host_start: 11,
        host_end: 15,
        host: HostInternal::Domain,
        port: None,
        path_start: 16,
        query_start: None,
        fragment_start: None,
    };

    let result = url.index(Position::AfterUsername);
}

#[test]
fn test_index_before_host() {
    let url = Url {
        serialization: String::from("http://localhost:80/path/to/resource"),
        scheme_end: 4,
        username_end: 4,
        host_start: 7,
        host_end: 14,
        host: HostInternal::Domain,
        port: Some(80),
        path_start: 15,
        query_start: None,
        fragment_start: None,
    };

    let result = url.index(Position::BeforeHost);
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
        path_start: 19,
        query_start: None,
        fragment_start: None,
    };

    let result = url.index(Position::AfterHost);
}

#[test]
fn test_index_before_port() {
    let url = Url {
        serialization: String::from("http://host:3000/path"),
        scheme_end: 4,
        username_end: 4,
        host_start: 7,
        host_end: 11,
        host: HostInternal::Domain,
        port: Some(3000),
        path_start: 12,
        query_start: None,
        fragment_start: None,
    };

    let result = url.index(Position::BeforePort);
} 

#[test]
fn test_index_after_port() {
    let url = Url {
        serialization: String::from("http://host:3000/path"),
        scheme_end: 4,
        username_end: 4,
        host_start: 7,
        host_end: 11,
        host: HostInternal::Domain,
        port: Some(3000),
        path_start: 12,
        query_start: None,
        fragment_start: None,
    };

    let result = url.index(Position::AfterPort);
}

#[test]
fn test_index_before_path() {
    let url = Url {
        serialization: String::from("http://host:8080/path/to/file"),
        scheme_end: 4,
        username_end: 4,
        host_start: 7,
        host_end: 11,
        host: HostInternal::Domain,
        port: Some(8080),
        path_start: 12,
        query_start: None,
        fragment_start: None,
    };

    let result = url.index(Position::BeforePath);
}

#[test]
fn test_index_after_path() {
    let url = Url {
        serialization: String::from("http://host/path/to/file"),
        scheme_end: 4,
        username_end: 4,
        host_start: 7,
        host_end: 11,
        host: HostInternal::Domain,
        port: None,
        path_start: 12,
        query_start: None,
        fragment_start: None,
    };

    let result = url.index(Position::AfterPath);
} 

#[test]
fn test_index_before_query() {
    let url = Url {
        serialization: String::from("http://host/path?query=value"),
        scheme_end: 4,
        username_end: 4,
        host_start: 7,
        host_end: 11,
        host: HostInternal::Domain,
        port: None,
        path_start: 12,
        query_start: Some(16),
        fragment_start: None,
    };

    let result = url.index(Position::BeforeQuery);
} 

#[test]
fn test_index_after_query() {
    let url = Url {
        serialization: String::from("http://host/path?query=value#fragment"),
        scheme_end: 4,
        username_end: 4,
        host_start: 7,
        host_end: 11,
        host: HostInternal::Domain,
        port: None,
        path_start: 12,
        query_start: Some(16),
        fragment_start: Some(24),
    };

    let result = url.index(Position::AfterQuery);
}

#[test]
fn test_index_before_fragment() {
    let url = Url {
        serialization: String::from("http://host/path?query=value#fragment"),
        scheme_end: 4,
        username_end: 4,
        host_start: 7,
        host_end: 11,
        host: HostInternal::Domain,
        port: None,
        path_start: 12,
        query_start: Some(16),
        fragment_start: Some(24),
    };

    let result = url.index(Position::BeforeFragment);
}

#[test]
fn test_index_after_fragment() {
    let url = Url {
        serialization: String::from("http://host/path#fragment"),
        scheme_end: 4,
        username_end: 4,
        host_start: 7,
        host_end: 11,
        host: HostInternal::Domain,
        port: None,
        path_start: 12,
        query_start: None,
        fragment_start: Some(18),
    };

    let result = url.index(Position::AfterFragment);
}

