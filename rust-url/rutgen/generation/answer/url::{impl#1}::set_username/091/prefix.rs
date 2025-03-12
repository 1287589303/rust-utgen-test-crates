// Answer 0

#[test]
fn test_set_username_valid() {
    let mut url = Url::parse("https://example.com?query=value#fragment").unwrap();
    url.set_username("newuser").unwrap();
}

#[test]
fn test_set_username_with_existing_username() {
    let mut url = Url::parse("http://user1@example.com?query=value#fragment").unwrap();
    url.set_username("user1").unwrap();
}

#[test]
fn test_set_username_change_username_with_at() {
    let mut url = Url::parse("ftp://user2:password@example.com?query=value#fragment").unwrap();
    url.set_username("user3").unwrap();
}

#[test]
fn test_set_username_change_username_with_colon() {
    let mut url = Url::parse("http://user:secret@example.com?query=value#fragment").unwrap();
    url.set_username("newuser").unwrap();
}

