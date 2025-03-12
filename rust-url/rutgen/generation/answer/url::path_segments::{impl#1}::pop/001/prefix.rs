// Answer 0

#[test]
fn test_pop_on_empty_path() {
    let mut url = Url {
        serialization: String::new(),
        scheme_end: 0,
        username_end: 0,
        host_start: 0,
        host_end: 0,
        host: HostInternal,
        port: None,
        path_start: 0,
        query_start: None,
        fragment_start: None,
    };

    let after_first_slash = url.serialization.len();

    let mut path_segments_mut = PathSegmentsMut {
        url: &mut url,
        after_first_slash,
        after_path: String::new(),
        old_after_path_position: 0,
    };

    path_segments_mut.pop();
}

#[test]
fn test_pop_on_path_only_scheme() {
    let mut url = Url {
        serialization: "http://".to_string(),
        scheme_end: 4,
        username_end: 0,
        host_start: 7,
        host_end: 7,
        host: HostInternal,
        port: None,
        path_start: 7,
        query_start: None,
        fragment_start: None,
    };

    let after_first_slash = url.serialization.len();

    let mut path_segments_mut = PathSegmentsMut {
        url: &mut url,
        after_first_slash,
        after_path: String::new(),
        old_after_path_position: 0,
    };

    path_segments_mut.pop();
}

