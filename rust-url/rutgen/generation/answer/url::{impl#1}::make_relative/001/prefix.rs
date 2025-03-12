// Answer 0

#[test]
fn test_make_relative_cannot_be_a_base_invalid_scheme() {
    let base = Url::parse("ftp://example.com").unwrap();
    let url = Url::parse("ftp://example.com/resource").unwrap();
    let relative = base.make_relative(&url);
}

#[test]
fn test_make_relative_cannot_be_a_base_no_authority() {
    let base = Url::parse("http://example.com/resource").unwrap();
    let url = Url::parse("http://example.com").unwrap();
    let relative = base.make_relative(&url);
}

#[test]
fn test_make_relative_cannot_be_a_base_mailto_scheme() {
    let base = Url::parse("mailto:user@example.com").unwrap();
    let url = Url::parse("mailto:user@example.com/email").unwrap();
    let relative = base.make_relative(&url);
}

