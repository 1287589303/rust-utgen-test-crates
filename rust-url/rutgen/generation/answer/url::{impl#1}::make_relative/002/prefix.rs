// Answer 0

#[test]
fn test_make_relative_scheme_different() {
    let base = Url::parse("http://example.net/a/b.html").unwrap();
    let url = Url::parse("https://example.net/a/c.png").unwrap();
    let relative = base.make_relative(&url);
}

#[test]
fn test_make_relative_scheme_different_with_query() {
    let base = Url::parse("ftp://example.net/a/b.html?param=value").unwrap();
    let url = Url::parse("http://example.net/a/c.png").unwrap();
    let relative = base.make_relative(&url);
}

#[test]
fn test_make_relative_scheme_different_with_fragment() {
    let base = Url::parse("http://example.net/a/b.html#section").unwrap();
    let url = Url::parse("https://example.net/a/c.png").unwrap();
    let relative = base.make_relative(&url);
}

