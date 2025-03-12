// Answer 0

#[test]
fn test_make_relative_case_1() {
    let base = Url::parse("https://example.com/").unwrap();
    let url = Url::parse("https://example.com/b/").unwrap();
    let relative = base.make_relative(&url);
}

#[test]
fn test_make_relative_with_query_and_fragment() {
    let base = Url::parse("https://example.com/a/").unwrap();
    let url = Url::parse("https://example.com/a/c.png?key=value#section").unwrap();
    let relative = base.make_relative(&url);
}

#[test]
fn test_make_relative_with_empty_base_path() {
    let base = Url::parse("https://example.com/a/b/").unwrap();
    let url = Url::parse("https://example.com/a/b/c.png?x=y#footer").unwrap();
    let relative = base.make_relative(&url);
}

#[test]
fn test_make_relative_directory_to_file() {
    let base = Url::parse("https://example.com/directory/").unwrap();
    let url = Url::parse("https://example.com/directory/file.txt?foo=bar#top").unwrap();
    let relative = base.make_relative(&url);
}

#[test]
fn test_make_relative_to_root() {
    let base = Url::parse("https://example.com/documents/").unwrap();
    let url = Url::parse("https://example.com/documents/index.html?view=full#page").unwrap();
    let relative = base.make_relative(&url);
}

