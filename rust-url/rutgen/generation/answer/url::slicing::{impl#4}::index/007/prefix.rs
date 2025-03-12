// Answer 0

#[test]
fn test_index_before_query_with_invalid_character() {
    let url = Url {
        serialization: String::from("http://username:password@host:8080/path"),
        scheme_end: 4,
        username_end: 12,
        host_start: 20,
        host_end: 24,
        host: HostInternal::Domain,
        port: Some(8080),
        path_start: 25,
        query_start: Some(30),
        fragment_start: None,
    };

    let position = Position::BeforeQuery;
    let result = url.index(position);
}

#[test]
fn test_index_before_query_with_invalid_character_and_no_fragment() {
    let url = Url {
        serialization: String::from("https://user:pass@localhost:8888/index.html"),
        scheme_end: 5,
        username_end: 12,
        host_start: 13,
        host_end: 21,
        host: HostInternal::Domain,
        port: Some(8888),
        path_start: 22,
        query_start: Some(27),
        fragment_start: None,
    };

    let position = Position::BeforeQuery;
    let result = url.index(position);
}

