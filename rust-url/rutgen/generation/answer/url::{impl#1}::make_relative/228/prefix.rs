// Answer 0

#[test]
fn test_make_relative_case1() {
    let base = Url::parse("https://example.com/path/to/resource")?;
    let url = Url::parse("https://example.com/path/to/resource/extra?query=param#fragment")?;
    let relative = base.make_relative(&url);
}

#[test]
fn test_make_relative_case2() {
    let base = Url::parse("https://example.com/another/path/to/resource/")?;
    let url = Url::parse("https://example.com/another/path/to/resource/extra?query=param#fragment")?;
    let relative = base.make_relative(&url);
}

#[test]
fn test_make_relative_case3() {
    let base = Url::parse("https://example.com/path/to/resource/")?;
    let url = Url::parse("https://example.com/path/to/resource/extra/file.txt?query=param#fragment")?;
    let relative = base.make_relative(&url);
}

#[test]
fn test_make_relative_case4() {
    let base = Url::parse("https://example.com/path/to/resource/a")?;
    let url = Url::parse("https://example.com/path/to/resource/a/b?query=param#fragment")?;
    let relative = base.make_relative(&url);
}

#[test]
fn test_make_relative_case5() {
    let base = Url::parse("https://example.com/path/to/resource/")?;
    let url = Url::parse("https://example.com/path/to/resource/another_folder/?query=param#fragment")?;
    let relative = base.make_relative(&url);
}

#[test]
fn test_make_relative_case6() {
    let base = Url::parse("https://example.com/path/to/resource")?;
    let url = Url::parse("https://example.com/path/to/resource?new=query#new_fragment")?;
    let relative = base.make_relative(&url);
}

