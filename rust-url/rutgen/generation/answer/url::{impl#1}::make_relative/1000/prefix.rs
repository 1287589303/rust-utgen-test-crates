// Answer 0

#[test]
fn test_make_relative_with_no_path_base_and_complex_target() {
    let base = Url::parse("https://example.com/").unwrap();
    let url = Url::parse("https://example.com/path/file.ext?query=value#fragment").unwrap();
    let relative = base.make_relative(&url);
}

#[test]
fn test_make_relative_with_empty_base_filename_and_non_empty_target() {
    let base = Url::parse("https://example.com/").unwrap();
    let url = Url::parse("https://example.com/other_path/file.html?query1=value1&query2=value2#section").unwrap();
    let relative = base.make_relative(&url);
}

#[test]
fn test_make_relative_with_base_as_directory_and_target_with_query_fragment() {
    let base = Url::parse("https://example.com/directory/").unwrap();
    let url = Url::parse("https://example.com/directory/file.png?size=large#image").unwrap();
    let relative = base.make_relative(&url);
}

