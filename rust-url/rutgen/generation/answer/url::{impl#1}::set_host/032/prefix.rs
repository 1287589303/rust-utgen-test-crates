// Answer 0

#[test]
fn test_set_host_change_host() {
    let mut url = Url::parse("ftp://example.com/path")?;
    let result = url.set_host(Some("new-example.com"));
    assert!(result.is_ok());
    assert_eq!(url.as_str(), "ftp://new-example.com/path");
}

#[test]
fn test_set_host_remove_host() {
    let mut url = Url::parse("http://example.com/path")?;
    let result = url.set_host(None);
    assert!(result.is_ok());
    assert_eq!(url.as_str(), "http:/path");
}

#[test]
fn test_set_host_invalid_host_format() {
    let mut url = Url::parse("http://example.com/path")?;
    let result = url.set_host(Some(":invalidhost"));
    assert!(result.is_err());
    assert_eq!(url.as_str(), "http://example.com/path");
}

#[test]
fn test_set_host_empty_host() {
    let mut url = Url::parse("http://example.com/path")?;
    let result = url.set_host(Some(""));
    assert!(result.is_err());
    assert_eq!(url.as_str(), "http://example.com/path");
}

#[test]
fn test_set_host_with_special_scheme() {
    let mut url = Url::parse("https://example.com/path")?;
    let result = url.set_host(Some("new-example.com"));
    assert!(result.is_ok());
    assert_eq!(url.as_str(), "https://new-example.com/path");
}

#[test]
fn test_set_host_change_host_with_path() {
    let mut url = Url::parse("http://example.com/old_path")?;
    let result = url.set_host(Some("new-example.com"));
    assert!(result.is_ok());
    assert_eq!(url.as_str(), "http://new-example.com/old_path");
}

