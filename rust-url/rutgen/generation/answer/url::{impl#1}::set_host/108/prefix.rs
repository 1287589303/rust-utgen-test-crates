// Answer 0

#[test]
fn test_set_host_success() {
    let mut url = Url::parse("https://example.com").unwrap();
    let result = url.set_host(Some("rust-lang.org"));
}

#[test]
fn test_set_host_with_subdomain() {
    let mut url = Url::parse("https://example.com").unwrap();
    let result = url.set_host(Some("sub.rust-lang.org"));
}

#[test]
fn test_set_host_with_numeric_host() {
    let mut url = Url::parse("https://example.com").unwrap();
    let result = url.set_host(Some("123.456.789.012"));
}

#[test]
fn test_set_host_with_long_host() {
    let mut url = Url::parse("https://example.com").unwrap();
    let result = url.set_host(Some("long.subdomain.rust-lang.org"));
}

#[test]
fn test_set_host_with_non_ascii() {
    let mut url = Url::parse("https://example.com").unwrap();
    let result = url.set_host(Some("rust-lang-测试.com"));
}

