// Answer 0

#[test]
fn test_set_query_with_existing_query_and_fragment() {
    let mut url = Url {
        serialization: "https://example.com/products#test".to_string(),
        scheme_end: 5,
        username_end: 0,
        host_start: 0,
        host_end: 15,
        host: HostInternal::Domain,
        port: None,
        path_start: 23,
        query_start: Some(24),
        fragment_start: Some(29),
    };
    url.set_query(Some("page=1"));
}

#[test]
fn test_set_query_with_fragment_present_and_replacing_query() {
    let mut url = Url {
        serialization: "https://example.com/products?old_query#fragment".to_string(),
        scheme_end: 5,
        username_end: 0,
        host_start: 0,
        host_end: 15,
        host: HostInternal::Domain,
        port: None,
        path_start: 23,
        query_start: Some(24),
        fragment_start: Some(36),
    };
    url.set_query(Some("sort=asc"));
}

#[test]
fn test_set_query_with_fragment_present_and_empty_query() {
    let mut url = Url {
        serialization: "https://example.com/products?old_query#fragment".to_string(),
        scheme_end: 5,
        username_end: 0,
        host_start: 0,
        host_end: 15,
        host: HostInternal::Domain,
        port: None,
        path_start: 23,
        query_start: Some(24),
        fragment_start: Some(36),
    };
    url.set_query(None);
}

#[test]
fn test_set_query_with_leading_trailing_spaces_and_fragment_present() {
    let mut url = Url {
        serialization: "https://example.com/products?old_query#test fragment".to_string(),
        scheme_end: 5,
        username_end: 0,
        host_start: 0,
        host_end: 15,
        host: HostInternal::Domain,
        port: None,
        path_start: 23,
        query_start: Some(24),
        fragment_start: Some(36),
    };
    url.set_query(Some("    search=test   "));
}

#[test]
fn test_set_query_with_multiple_parameters_and_fragment_present() {
    let mut url = Url {
        serialization: "https://example.com/products?old_query#test".to_string(),
        scheme_end: 5,
        username_end: 0,
        host_start: 0,
        host_end: 15,
        host: HostInternal::Domain,
        port: None,
        path_start: 23,
        query_start: Some(24),
        fragment_start: Some(36),
    };
    url.set_query(Some("page=2&sort=asc"));
}

