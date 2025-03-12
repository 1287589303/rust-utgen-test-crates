// Answer 0

#[test]
fn test_scheme_valid_http() {
    let url = Url::parse("http://example.com").unwrap();
    let _ = url.scheme();
}

#[test]
fn test_scheme_valid_https() {
    let url = Url::parse("https://example.com").unwrap();
    let _ = url.scheme();
}

#[test]
fn test_scheme_valid_ftp() {
    let url = Url::parse("ftp://ftp.example.com").unwrap();
    let _ = url.scheme();
}

#[test]
fn test_scheme_valid_file() {
    let url = Url::parse("file:///tmp/foo").unwrap();
    let _ = url.scheme();
}

#[test]
fn test_scheme_malformed() {
    let result = Url::parse("://invalid");
    assert!(result.is_err());
}

#[test]
fn test_scheme_empty_string() {
    let result = Url::parse("");
    assert!(result.is_err());
}

#[test]
fn test_scheme_no_scheme() {
    let url = Url::parse("example.com").unwrap();
    let _ = url.scheme();
}

