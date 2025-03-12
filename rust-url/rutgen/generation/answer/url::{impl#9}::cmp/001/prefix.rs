// Answer 0

#[test]
fn test_cmp_equal_strings() {
    let url1 = Url {
        serialization: String::from("http://example.com"),
        scheme_end: 4,
        username_end: 0,
        host_start: 7,
        host_end: 11,
        host: HostInternal::Domain,
        port: None,
        path_start: 0,
        query_start: None,
        fragment_start: None,
    };
    
    let url2 = Url {
        serialization: String::from("http://example.com"),
        scheme_end: 4,
        username_end: 0,
        host_start: 7,
        host_end: 11,
        host: HostInternal::Domain,
        port: None,
        path_start: 0,
        query_start: None,
        fragment_start: None,
    };
    
    let _result = url1.cmp(&url2);
}

#[test]
fn test_cmp_different_strings() {
    let url1 = Url {
        serialization: String::from("http://example.com"),
        scheme_end: 4,
        username_end: 0,
        host_start: 7,
        host_end: 11,
        host: HostInternal::Domain,
        port: None,
        path_start: 0,
        query_start: None,
        fragment_start: None,
    };
    
    let url2 = Url {
        serialization: String::from("https://example.com"),
        scheme_end: 5,
        username_end: 0,
        host_start: 8,
        host_end: 11,
        host: HostInternal::Domain,
        port: None,
        path_start: 0,
        query_start: None,
        fragment_start: None,
    };
    
    let _result = url1.cmp(&url2);
}

#[test]
fn test_cmp_strings_with_one_char_difference() {
    let url1 = Url {
        serialization: String::from("http://example.com/page"),
        scheme_end: 4,
        username_end: 0,
        host_start: 7,
        host_end: 11,
        host: HostInternal::Domain,
        port: None,
        path_start: 0,
        query_start: None,
        fragment_start: None,
    };
    
    let url2 = Url {
        serialization: String::from("http://example.com/pag"),
        scheme_end: 4,
        username_end: 0,
        host_start: 7,
        host_end: 11,
        host: HostInternal::Domain,
        port: None,
        path_start: 0,
        query_start: None,
        fragment_start: None,
    };
    
    let _result = url1.cmp(&url2);
}

#[test]
fn test_cmp_empty_strings() {
    let url1 = Url {
        serialization: String::new(),
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
    
    let url2 = Url {
        serialization: String::new(),
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
    
    let _result = url1.cmp(&url2);
}

#[test]
fn test_cmp_large_strings() {
    let long_str1 = "http://" + &"a".repeat(1_000);
    let long_str2 = "http://" + &"b".repeat(1_000);
    
    let url1 = Url {
        serialization: long_str1,
        scheme_end: 4,
        username_end: 0,
        host_start: 7,
        host_end: 1007,
        host: HostInternal::Domain,
        port: None,
        path_start: 0,
        query_start: None,
        fragment_start: None,
    };
    
    let url2 = Url {
        serialization: long_str2,
        scheme_end: 4,
        username_end: 0,
        host_start: 7,
        host_end: 1007,
        host: HostInternal::Domain,
        port: None,
        path_start: 0,
        query_start: None,
        fragment_start: None,
    };
    
    let _result = url1.cmp(&url2);
}

