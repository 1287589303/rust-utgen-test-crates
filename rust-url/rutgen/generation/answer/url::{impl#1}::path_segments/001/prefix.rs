// Answer 0

#[test]
fn test_valid_path_segments_with_multiple_segments() {
    let url = Url::parse("https://example.com/foo/bar").unwrap();
    let _ = url.path_segments();
}

#[test]
fn test_valid_path_segments_with_empty_last_segment() {
    let url = Url::parse("https://example.com/path/").unwrap();
    let _ = url.path_segments();
}

#[test]
fn test_valid_path_segments_with_single_empty_segment() {
    let url = Url::parse("https://example.com").unwrap();
    let _ = url.path_segments();
}

#[test]
fn test_invalid_url_cannot_be_base() {
    let url = Url::parse("data:text/plain,HelloWorld").unwrap();
    let _ = url.path_segments();
}

#[test]
fn test_valid_path_segments_with_spaces() {
    let url = Url::parse("https://example.com/path with spaces").unwrap();
    let _ = url.path_segments();
}

#[test]
fn test_invalid_url_malformed() {
    let url = Url::parse("abc://invalid:url").unwrap();
    let _ = url.path_segments();
}

#[test]
fn test_invalid_url_no_path_scheme_only() {
    let url = Url::parse("http:///invalid").unwrap();
    let _ = url.path_segments();
}

#[test]
fn test_valid_path_segments_with_unicode() {
    let url = Url::parse("https://example.com/countries/việt nam").unwrap();
    let _ = url.path_segments();
}

