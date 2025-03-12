// Answer 0

#[test]
fn test_username_no_authority_http() {
    let url = Url::parse("http://example.com").unwrap();
    let _ = url.username();
}

#[test]
fn test_username_no_authority_ftp() {
    let url = Url::parse("ftp://example.com").unwrap();
    let _ = url.username();
}

#[test]
fn test_username_no_authority_https() {
    let url = Url::parse("https://example.com").unwrap();
    let _ = url.username();
}

#[test]
fn test_username_no_authority_path() {
    let url = Url::parse("https://example.com/path").unwrap();
    let _ = url.username();
}

#[test]
fn test_username_no_authority_file() {
    let url = Url::parse("file:///path/to/file").unwrap();
    let _ = url.username();
}

