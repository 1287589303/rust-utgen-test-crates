// Answer 0

#[test]
fn test_password_with_username_and_password_containing_colon() {
    let url = Url::parse("ftp://user:pass:word@example.com").unwrap();
    let _ = url.password();
}

#[test]
fn test_password_with_password_only() {
    let url = Url::parse("ftp://:secret@example.com").unwrap();
    let _ = url.password();
}

#[test]
fn test_password_without_username() {
    let url = Url::parse("ftp://user@example.com").unwrap();
    let _ = url.password();
}

#[test]
fn test_password_without_authority() {
    let url = Url::parse("https://example.com").unwrap();
    let _ = url.password();
}

