// Answer 0

#[test]
fn test_query_with_valid_ascii() {
    let url = Url::parse("https://example.com/products?page=2").unwrap();
    let query = url.query();
}

#[test]
fn test_query_with_percent_encoding() {
    let url = Url::parse("https://example.com/?country=español").unwrap();
    let query = url.query();
}

#[test]
fn test_query_no_fragment() {
    let url = Url::parse("https://example.com/products").unwrap();
    let query = url.query();
}

