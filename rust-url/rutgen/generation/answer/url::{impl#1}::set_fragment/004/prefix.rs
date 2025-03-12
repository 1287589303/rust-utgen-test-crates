// Answer 0

#[test]
fn test_set_fragment_invalid_start() {
    let mut url = Url {
        serialization: String::from("https://example.com/data.csv"),
        scheme_end: 5,
        username_end: 0,
        host_start: 0,
        host_end: 0,
        host: HostInternal::None,
        port: None,
        path_start: 0,
        query_start: None,
        fragment_start: Some(0), // valid start index
    };

    // Set an invalid fragment where byte_at(start) == b'#'
    url.serialization.push('#'); // Add a '#' at the end
    url.set_fragment(Some("cell=4,1-6,2")); // Input should cause fragment_start to be updated
} 

#[test]
fn test_set_fragment_with_invalid_byte_at_start() {
    let mut url = Url {
        serialization: String::from("https://example.com/data.csv"),
        scheme_end: 5,
        username_end: 0,
        host_start: 0,
        host_end: 0,
        host: HostInternal::None,
        port: None,
        path_start: 0,
        query_start: None,
        fragment_start: Some(2), // valid start index
    };

    url.serialization.push('a'); // Make sure that the byte at index 2 is not '#'
    url.set_fragment(Some("new_fragment")); // Set a new fragment
} 

#[test]
fn test_set_fragment_with_non_empty_string() {
    let mut url = Url {
        serialization: String::from("https://example.com/data.csv"),
        scheme_end: 5,
        username_end: 0,
        host_start: 0,
        host_end: 0,
        host: HostInternal::None,
        port: None,
        path_start: 0,
        query_start: None,
        fragment_start: Some(5), // valid start index
    };

    url.set_fragment(Some("valid_fragment")); // Set a non-empty fragment
}

