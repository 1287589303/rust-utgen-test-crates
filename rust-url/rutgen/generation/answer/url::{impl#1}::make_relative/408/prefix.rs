// Answer 0

#[test]
fn test_make_relative_case1() {
    let base = Url::parse("https://example.com/a/b.html?foo=bar#section")?;
    let url = Url::parse("https://example.com/a/c.png?baz=qux#fragment")?;
    let relative = base.make_relative(&url);
}

#[test]
fn test_make_relative_case2() {
    let base = Url::parse("https://example.com/a/b/")?;
    let url = Url::parse("https://example.com/a/b/c.png?param=value#frag")?;
    let relative = base.make_relative(&url);
}

#[test]
fn test_make_relative_case3() {
    let base = Url::parse("https://example.com/a/b/")?;
    let url = Url::parse("https://example.com/a/d/c.png?abc=def#header")?;
    let relative = base.make_relative(&url);
}

#[test]
fn test_make_relative_case4() {
    let base = Url::parse("https://example.com/a/b.html?c=d#top")?;
    let url = Url::parse("https://example.com/a/b.html?e=f#bottom")?;
    let relative = base.make_relative(&url);
}

