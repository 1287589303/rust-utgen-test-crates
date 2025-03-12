// Answer 0

#[test]
fn test_password_with_username_and_password() {
    let url = Url::parse("http://user:pass@hostname.com").unwrap();
    let _ = url.password();
}

#[test]
fn test_password_with_empty_username() {
    let url = Url::parse("http://:pass@hostname.com").unwrap();
    let _ = url.password();
}

#[test]
fn test_password_with_only_username() {
    let url = Url::parse("http://user@hostname.com").unwrap();
    let _ = url.password();
}

#[test]
fn test_password_without_username_or_password() {
    let url = Url::parse("http://hostname.com").unwrap();
    let _ = url.password();
}

