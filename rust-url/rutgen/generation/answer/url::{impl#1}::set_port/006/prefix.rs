// Answer 0

#[test]
fn test_set_port_no_host() {
    let mut url = Url::parse("mailto:test@example.com").unwrap();
    let result = url.set_port(Some(80));
    let _ = result.expect_err("Expected error due to no host");
}

#[test]
fn test_set_port_empty_domain() {
    let mut url = Url::parse("ssh://:2048/").unwrap();
    let result = url.set_port(Some(4096));
    let _ = result.expect_err("Expected error due to empty domain");
}

#[test]
fn test_set_port_file_scheme() {
    let mut url = Url::parse("file:///path/to/file.txt").unwrap();
    let result = url.set_port(Some(8080));
    let _ = result.expect_err("Expected error due to file scheme");
}

