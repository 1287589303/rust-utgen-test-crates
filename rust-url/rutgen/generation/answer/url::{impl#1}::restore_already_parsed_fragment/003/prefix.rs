// Answer 0

#[test]
fn test_restore_already_parsed_fragment_non_empty() {
    let mut url = Url {
        serialization: String::from("http://example.com#existing_fragment"),
        scheme_end: 4,
        username_end: 0,
        host_start: 7,
        host_end: 14,
        host: HostInternal::Domain,
        port: None,
        path_start: 0,
        query_start: None,
        fragment_start: Some(25),
    };
    let fragment = Some(String::from("new_fragment"));
    url.restore_already_parsed_fragment(fragment);
}

#[test]
fn test_restore_already_parsed_fragment_with_different_fragment() {
    let mut url = Url {
        serialization: String::from("https://example.com#another_fragment"),
        scheme_end: 5,
        username_end: 0,
        host_start: 8,
        host_end: 15,
        host: HostInternal::Domain,
        port: None,
        path_start: 0,
        query_start: None,
        fragment_start: Some(26),
    };
    let fragment = Some(String::from("changed_fragment"));
    url.restore_already_parsed_fragment(fragment);
}

#[test]
fn test_restore_already_parsed_fragment_with_special_characters() {
    let mut url = Url {
        serialization: String::from("ftp://example.com#old_fragment"),
        scheme_end: 3,
        username_end: 0,
        host_start: 6,
        host_end: 13,
        host: HostInternal::Domain,
        port: None,
        path_start: 0,
        query_start: None,
        fragment_start: Some(24),
    };
    let fragment = Some(String::from("fragment_with_spaces and special*&chars"));
    url.restore_already_parsed_fragment(fragment);
}

#[test]
fn test_restore_already_parsed_fragment_with_numeric() {
    let mut url = Url {
        serialization: String::from("mailto:user@example.com#previous_value"),
        scheme_end: 6,
        username_end: 0,
        host_start: 7,
        host_end: 18,
        host: HostInternal::Domain,
        port: None,
        path_start: 0,
        query_start: None,
        fragment_start: Some(29),
    };
    let fragment = Some(String::from("123456"));
    url.restore_already_parsed_fragment(fragment);
}

