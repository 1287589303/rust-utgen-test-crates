// Answer 0

#[test]
fn test_is_special_http() {
    let url = Url::parse("http://example.com").unwrap();
    url.is_special();
}

#[test]
fn test_is_special_https() {
    let url = Url::parse("https://secure-site.com").unwrap();
    url.is_special();
}

#[test]
fn test_is_special_file() {
    let url = Url::parse("file:///tmp/foo").unwrap();
    url.is_special();
}

#[test]
fn test_is_special_ftp() {
    let url = Url::parse("ftp://files.example.com").unwrap();
    url.is_special();
}

#[test]
fn test_is_special_moz() {
    let url = Url::parse("moz://example.com").unwrap();
    url.is_special();
}

#[test]
fn test_is_special_empty() {
    let url = Url::parse("").unwrap_err();
}

#[test]
fn test_is_special_invalid_url() {
    let url = Url::parse("invalid-url").unwrap_err();
}

#[test]
fn test_is_special_malformed_http() {
    let url = Url::parse("http:///tmp/foo").unwrap();
    url.is_special();
}

#[test]
fn test_is_special_non_special() {
    let url = Url::parse("customscheme://path").unwrap();
    url.is_special();
}

#[test]
fn test_is_special_special_encoding() {
    let url = Url::parse("http://example.com/path?query=value#fragment").unwrap();
    url.is_special();
}

