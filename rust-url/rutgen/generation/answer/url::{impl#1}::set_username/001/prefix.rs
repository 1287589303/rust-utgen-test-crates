// Answer 0

#[test]
fn test_set_username_with_empty_domain() {
    let mut url = Url::parse("file://:@example.com").unwrap();
    let result = url.set_username("user1");
}

#[test]
fn test_set_username_with_empty_domain_other_scheme() {
    let mut url = Url::parse("http://:@example.com").unwrap();
    let result = url.set_username("user1");
}

#[test]
fn test_set_username_with_empty_domain_ftp_scheme() {
    let mut url = Url::parse("ftp://:@example.com").unwrap();
    let result = url.set_username("user1");
}

