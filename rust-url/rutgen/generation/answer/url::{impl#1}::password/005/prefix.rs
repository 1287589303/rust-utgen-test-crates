// Answer 0

#[test]
fn test_password_without_authority_no_username() {
    let url = Url::parse("ftp://example.com").unwrap();
    url.password();
}

#[test]
fn test_password_without_authority_with_port() {
    let url = Url::parse("http://example.com:80").unwrap();
    url.password();
}

#[test]
fn test_password_without_authority_file_url() {
    let url = Url::parse("file:///path/to/file").unwrap();
    url.password();
}

#[test]
fn test_password_without_authority_empty_username() {
    let url = Url::parse("ftp://:@example.com").unwrap();
    url.password();
}

