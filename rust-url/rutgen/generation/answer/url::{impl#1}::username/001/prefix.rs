// Answer 0

#[test]
fn test_username_with_valid_username() {
    let url = Url::parse("http://user:pass@host.com").unwrap();
    let username = url.username();
}

#[test]
fn test_username_with_special_characters() {
    let url = Url::parse("ftp://user%20name@server.com").unwrap();
    let username = url.username();
}

#[test]
fn test_username_with_digits() {
    let url = Url::parse("http://user123:secret@host.com").unwrap();
    let username = url.username();
}

#[test]
fn test_username_with_empty_password() {
    let url = Url::parse("http://username:@host.com").unwrap();
    let username = url.username();
}

#[test]
fn test_username_with_encoding() {
    let url = Url::parse("http://user%3Apass@host.com").unwrap();
    let username = url.username();
}

