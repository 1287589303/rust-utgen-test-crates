// Answer 0

#[test]
fn test_new_special_scheme_with_non_empty_path() {
    let mut url = Url {
        serialization: String::from("special://path"),
        scheme_end: 8,
        username_end: 0,
        host_start: 0,
        host_end: 0,
        host: HostInternal::default(),
        port: None,
        path_start: 8,
        query_start: None,
        fragment_start: None,
    };
    let _path_segments = new(&mut url);
}

#[test]
fn test_new_special_scheme_with_single_segment_path() {
    let mut url = Url {
        serialization: String::from("special://segment"),
        scheme_end: 8,
        username_end: 0,
        host_start: 0,
        host_end: 0,
        host: HostInternal::default(),
        port: None,
        path_start: 8,
        query_start: None,
        fragment_start: None,
    };
    let _path_segments = new(&mut url);
}

#[test]
fn test_new_special_scheme_with_multiple_segments_path() {
    let mut url = Url {
        serialization: String::from("special://segment1/segment2"),
        scheme_end: 8,
        username_end: 0,
        host_start: 0,
        host_end: 0,
        host: HostInternal::default(),
        port: None,
        path_start: 8,
        query_start: None,
        fragment_start: None,
    };
    let _path_segments = new(&mut url);
}

#[test]
fn test_new_special_scheme_with_path_containing_query() {
    let mut url = Url {
        serialization: String::from("special://path?query=value"),
        scheme_end: 8,
        username_end: 0,
        host_start: 0,
        host_end: 0,
        host: HostInternal::default(),
        port: None,
        path_start: 8,
        query_start: Some(12),
        fragment_start: None,
    };
    let _path_segments = new(&mut url);
}

#[test]
fn test_new_special_scheme_with_path_containing_fragment() {
    let mut url = Url {
        serialization: String::from("special://path#fragment"),
        scheme_end: 8,
        username_end: 0,
        host_start: 0,
        host_end: 0,
        host: HostInternal::default(),
        port: None,
        path_start: 8,
        query_start: None,
        fragment_start: Some(12),
    };
    let _path_segments = new(&mut url);
}

