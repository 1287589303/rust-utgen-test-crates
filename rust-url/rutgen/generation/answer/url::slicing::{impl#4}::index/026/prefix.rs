// Answer 0

#[test]
fn test_index_after_password_with_authority() {
    let url = Url {
        serialization: String::from("http://user:pass@host:8080/path?query#fragment"),
        scheme_end: 4,
        username_end: 8,
        host_start: 9,
        host_end: 13,
        port: Some(8080),
        path_start: 17,
        query_start: Some(22),
        fragment_start: Some(28),
    };
    let position = Position::AfterPassword;
    let _ = url.index(position);
}

#[test]
fn test_index_after_password_with_different_user() {
    let url = Url {
        serialization: String::from("https://username:password@another-host:3000/somepath?query#frag"),
        scheme_end: 5,
        username_end: 9,
        host_start: 10,
        host_end: 21,
        port: Some(3000),
        path_start: 22,
        query_start: Some(31),
        fragment_start: Some(36),
    };
    let position = Position::AfterPassword;
    let _ = url.index(position);
}

