// Answer 0

#[test]
fn test_set_password_has_host_empty_domain() {
    let mut url = Url::parse("ftp://:@example.com").unwrap();
    let result = url.set_password(Some("new_password"));
}

#[test]
fn test_set_password_has_host_empty_domain_none() {
    let mut url = Url::parse("ftp://:@example.com").unwrap();
    let result = url.set_password(None);
}

