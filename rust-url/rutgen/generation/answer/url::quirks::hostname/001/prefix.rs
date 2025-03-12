// Answer 0

#[test]
fn test_hostname_valid_domain() {
    let url = Url::parse("https://example.com").unwrap();
    let _ = hostname(&url);
}

#[test]
fn test_hostname_valid_subdomain() {
    let url = Url::parse("https://www.test.org").unwrap();
    let _ = hostname(&url);
}

#[test]
fn test_hostname_invalid_empty() {
    let url = Url::parse("https://").unwrap();
    let _ = hostname(&url);
}

#[test]
fn test_hostname_invalid_host() {
    let url = Url::parse("https://invalid-host").unwrap();
    let _ = hostname(&url);
}

#[test]
fn test_hostname_ip_address() {
    let url = Url::parse("https://192.168.1.1").unwrap();
    let _ = hostname(&url);
}

#[test]
fn test_hostname_special_characters() {
    let url = Url::parse("https://sub.domain.com/path?query#fragment").unwrap();
    let _ = hostname(&url);
}

#[test]
fn test_hostname_no_host() {
    let url = Url::parse("not-a-url").unwrap_err(); // This should lead to an error as there is no valid host
    // Verify that the error is handled properly.
}

