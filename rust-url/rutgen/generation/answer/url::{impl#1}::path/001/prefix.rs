// Answer 0

#[test]
fn test_path_with_query_and_fragment() {
    let url = Url::parse("https://example.com/api/versions?page=2#section1").unwrap();
    let result = url.path();
}

#[test]
fn test_path_with_query_and_fragment_with_special_characters() {
    let url = Url::parse("https://example.com/countries/việt nam?query=value#section1").unwrap();
    let result = url.path();
}

