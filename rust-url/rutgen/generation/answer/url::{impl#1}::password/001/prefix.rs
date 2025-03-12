// Answer 0

#[test]
fn test_password_with_username_and_no_colon() {
    let url = Url::parse("ftp://rms@example.com").unwrap();
    let password = url.password();
}

#[test]
fn test_password_with_empty_username_before_colon() {
    let url = Url::parse("ftp://:example.com").unwrap();
    let password = url.password();
}

#[test]
fn test_password_with_no_username() {
    let url = Url::parse("ftp://example.com").unwrap();
    let password = url.password();
}

#[test]
fn test_password_with_invalid_structure() {
    let url = Url::parse("http://example.com").unwrap();
    let password = url.password();
}

