// Answer 0

#[test]
fn test_password_no_username() {
    let url = Url::parse("ftp://:secret123@example.com").unwrap(); // username_end == serialization length
    let _ = url.password();
}

#[test]
fn test_password_no_username_https() {
    let url = Url::parse("https://:password@example.com").unwrap(); // username_end == serialization length
    let _ = url.password();
}

#[test]
fn test_password_no_username_http() {
    let url = Url::parse("http://:mypassword@example.com").unwrap(); // username_end == serialization length
    let _ = url.password();
}

#[test]
fn test_password_no_username_with_trailing_slash() {
    let url = Url::parse("http://:password@example.com/").unwrap(); // username_end == serialization length
    let _ = url.password();
}

