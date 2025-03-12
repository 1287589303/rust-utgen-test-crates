// Answer 0

#[test]
fn test_extend_valid_segments_no_slashes() {
    let mut url = Url::parse("https://example.com/").unwrap();
    url.path_segments_mut().unwrap().extend(&["foo", "bar", "baz"]);
}

#[test]
fn test_extend_valid_segments_with_trailing_slashes() {
    let mut url = Url::parse("https://example.com/foo/").unwrap();
    url.path_segments_mut().unwrap().extend(&["bar/", "baz/"]);
}

#[test]
fn test_extend_empty_path_segments() {
    let mut url = Url::parse("https://example.com/").unwrap();
    url.path_segments_mut().unwrap().extend(&["", "", ""]);
}

#[test]
fn test_extend_segments_with_percent_and_slash() {
    let mut url = Url::parse("https://example.com/").unwrap();
    url.path_segments_mut().unwrap().extend(&["foo%20bar", "baz%2Fqux"]);
}

#[test]
fn test_extend_segments_with_special_characters() {
    let mut url = Url::parse("https://example.com/").unwrap();
    url.path_segments_mut().unwrap().extend(&["foo@bar", "baz#qux"]);
}

#[test]
fn test_extend_ignore_dot_segments() {
    let mut url = Url::parse("https://example.com/").unwrap();
    url.path_segments_mut().unwrap().extend(&["..", ".", "validSegment"]);
}

#[test]
fn test_extend_valid_empty_segments() {
    let mut url = Url::parse("https://example.com/").unwrap();
    url.path_segments_mut().unwrap().extend(&["", "validSegment"]);
}

#[test]
fn test_extend_existing_path_is_slash() {
    let mut url = Url::parse("https://example.com/").unwrap();
    url.path_segments_mut().unwrap().extend(&["newSegment"]);
}

#[test]
fn test_extend_combination_valid_and_invalid_segments() {
    let mut url = Url::parse("https://example.com/foo").unwrap();
    url.path_segments_mut().unwrap().extend(&["..", "bar", ".", "baz", ""]);
}

#[test]
fn test_extend_mixture_valid_and_empty_segments() {
    let mut url = Url::parse("https://example.com/").unwrap();
    url.path_segments_mut().unwrap().extend(&["valid", "", "segment"]);
}

