// Answer 0

#[test]
fn test_make_relative_valid_case_1() {
    let base = Url::parse("https://example.com/path/to/dir/")?;
    let url = Url::parse("https://example.com/path/to/file.png?query=value#fragment")?;
    let relative = base.make_relative(&url);
}

#[test]
fn test_make_relative_valid_case_2() {
    let base = Url::parse("https://example.com/images/")?;
    let url = Url::parse("https://example.com/images/photo.png?size=large#gallery")?;
    let relative = base.make_relative(&url);
}

#[test]
fn test_make_relative_valid_case_3() {
    let base = Url::parse("https://example.com/test/")?;
    let url = Url::parse("https://example.com/test/file.txt?param=value#section")?;
    let relative = base.make_relative(&url);
}

#[test]
fn test_make_relative_valid_case_4() {
    let base = Url::parse("https://example.com/docs/")?;
    let url = Url::parse("https://example.com/docs/manual.pdf?version=1.2#cover")?;
    let relative = base.make_relative(&url);
}

#[test]
fn test_make_relative_valid_case_5() {
    let base = Url::parse("https://example.com/resources/")?;
    let url = Url::parse("https://example.com/resources/docs/guide.pdf?lang=en#intro")?;
    let relative = base.make_relative(&url);
}

