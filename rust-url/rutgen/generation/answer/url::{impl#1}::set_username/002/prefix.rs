// Answer 0

#[test]
fn test_set_username_file_scheme_no_host() {
    let mut url = Url::parse("file:///").unwrap();
    let result = url.set_username("user1");
}

#[test]
fn test_set_username_file_scheme_no_authority() {
    let mut url = Url::parse("file://").unwrap();
    let result = url.set_username("user1");
}

#[test]
fn test_set_username_file_scheme_empty_username() {
    let mut url = Url::parse("file:///").unwrap();
    let result = url.set_username("");
}

