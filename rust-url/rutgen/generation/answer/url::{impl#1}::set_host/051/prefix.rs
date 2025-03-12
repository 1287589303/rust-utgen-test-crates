// Answer 0

#[test]
fn test_set_host_change_special_scheme_with_empty_host() {
    let mut url = Url::parse("file://example.net").unwrap(); // Valid scheme (file)
    let result = url.set_host(Some("[rust-lang.org")); // Invalid hostname ends with [
    assert!(result.is_err());
}

#[test]
fn test_set_host_change_special_scheme_with_empty_host_and_colon() {
    let mut url = Url::parse("http://example.net").unwrap(); // Special scheme (http)
    let result = url.set_host(Some("[rust-lang.org:")); // Invalid hostname ending without ]
    assert!(result.is_err());
}

#[test]
fn test_set_host_remove_host_in_file_scheme() {
    let mut url = Url::parse("file://example.net").unwrap(); // Valid scheme (file)
    let result = url.set_host(None); // Removing the host from a file scheme
    assert!(result.is_ok());
    assert_eq!(url.as_str(), "file:/"); // Expected outcome after removing host
}

#[test]
fn test_set_host_change_host_in_special_scheme() {
    let mut url = Url::parse("http://example.net").unwrap(); // Special scheme (http)
    let result = url.set_host(Some("[new-host.com]")); // Changing to new host with brackets
    assert!(result.is_ok());
    assert_eq!(url.as_str(), "http://new-host.com/"); // Expected outcome after changing host
}

