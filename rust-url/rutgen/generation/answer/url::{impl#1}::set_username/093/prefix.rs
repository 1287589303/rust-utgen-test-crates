// Answer 0

#[test]
fn test_set_username_no_host_mailto() {
    let mut url = Url::parse("mailto:rmz@example.com").unwrap();
    let result = url.set_username("user1");
}

#[test]
fn test_set_username_no_host_file() {
    let mut url = Url::parse("file:///path/to/file").unwrap();
    let result = url.set_username("user1");
}

#[test]
fn test_set_username_no_host_empty_username() {
    let mut url = Url::parse("ftp://:secre1@example.com/").unwrap();
    let result = url.set_username("");
}

