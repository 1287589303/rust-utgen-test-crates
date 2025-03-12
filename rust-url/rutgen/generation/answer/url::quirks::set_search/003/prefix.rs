// Answer 0

#[test]
fn test_set_search_with_valid_string() {
    let mut url = Url::parse("http://example.com").unwrap();
    let new_search = "query=1";
    set_search(&mut url, new_search);
}

#[test]
fn test_set_search_with_valid_string_with_space() {
    let mut url = Url::parse("http://example.com").unwrap();
    let new_search = "search term";
    set_search(&mut url, new_search);
}

#[test]
fn test_set_search_with_valid_numeric_string() {
    let mut url = Url::parse("http://example.com").unwrap();
    let new_search = "12345";
    set_search(&mut url, new_search);
}

#[test]
fn test_set_search_with_valid_special_characters() {
    let mut url = Url::parse("http://example.com").unwrap();
    let new_search = "foo@bar";
    set_search(&mut url, new_search);
}

