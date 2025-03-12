// Answer 0

#[test]
fn test_set_username_username_is_equal_http() {
    let mut url = Url::parse("http://user:pass@example.com/").unwrap();
    let result = url.set_username("user");
}

#[test]
fn test_set_username_username_is_equal_ftp() {
    let mut url = Url::parse("ftp://user:pass@example.com/").unwrap();
    let result = url.set_username("user");
}

#[test]
fn test_set_username_username_is_equal_https() {
    let mut url = Url::parse("https://user:secret@example.com/").unwrap();
    let result = url.set_username("user");
}

#[test]
fn test_set_username_username_is_equal_custom_scheme() {
    let mut url = Url::parse("customscheme://user:token@domain.com/").unwrap();
    let result = url.set_username("user");
}

