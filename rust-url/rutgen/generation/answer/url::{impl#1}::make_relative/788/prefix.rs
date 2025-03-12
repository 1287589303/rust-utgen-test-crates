// Answer 0

#[test]
fn test_make_relative_with_non_empty_base_and_url() {
    let base = Url::parse("https://example.com/path/to/base/")?;
    let url = Url::parse("https://example.com/path/to/base/file.txt?query=1#fragment")?;
    let relative = base.make_relative(&url);
}

#[test]
fn test_make_relative_with_directory_and_file() {
    let base = Url::parse("https://example.com/path/to/dir/")?;
    let url = Url::parse("https://example.com/path/to/dir/file.png?arg=value#section")?;
    let relative = base.make_relative(&url);
}

#[test]
fn test_make_relative_with_common_path_and_different_file() {
    let base = Url::parse("https://example.com/path/to/")?;
    let url = Url::parse("https://example.com/path/to/anotherfile.html?key=value#part")?;
    let relative = base.make_relative(&url);
}

#[test]
fn test_make_relative_with_query_and_fragment() {
    let base = Url::parse("https://example.com/path/to/resource/?foo=bar")?;
    let url = Url::parse("https://example.com/path/to/resource/other.txt?baz=qux#anchor")?;
    let relative = base.make_relative(&url);
}

#[test]
fn test_make_relative_with_multiple_segments() {
    let base = Url::parse("https://example.com/a/b/c/d/")?;
    let url = Url::parse("https://example.com/a/b/x/y/z?search=test#top")?;
    let relative = base.make_relative(&url);
}

