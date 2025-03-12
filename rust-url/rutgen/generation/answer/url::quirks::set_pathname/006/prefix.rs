// Answer 0

#[test] 
fn test_set_pathname_case_1() { 
    let mut url = Url { 
        serialization: "http://example.com".to_string(), 
        scheme_end: 4, 
        username_end: 0, 
        host_start: 7, 
        host_end: 14, 
        host: HostInternal::Some("example.com".to_string()), 
        port: None, 
        path_start: 0, 
        query_start: None, 
        fragment_start: None 
    }; 
    let new_pathname = ""; 
    set_pathname(&mut url, new_pathname); 
}

#[test] 
fn test_set_pathname_case_2() { 
    let mut url = Url { 
        serialization: "http://localhost".to_string(), 
        scheme_end: 4, 
        username_end: 0, 
        host_start: 7, 
        host_end: 12, 
        host: HostInternal::Some("localhost".to_string()), 
        port: None, 
        path_start: 0, 
        query_start: None, 
        fragment_start: None 
    }; 
    let new_pathname = ""; 
    set_pathname(&mut url, new_pathname); 
}

#[test] 
fn test_set_pathname_case_3() { 
    let mut url = Url { 
        serialization: "https://example.org".to_string(), 
        scheme_end: 5, 
        username_end: 0, 
        host_start: 8, 
        host_end: 16, 
        host: HostInternal::Some("example.org".to_string()), 
        port: None, 
        path_start: 0, 
        query_start: None, 
        fragment_start: None 
    }; 
    let new_pathname = ""; 
    set_pathname(&mut url, new_pathname); 
}

