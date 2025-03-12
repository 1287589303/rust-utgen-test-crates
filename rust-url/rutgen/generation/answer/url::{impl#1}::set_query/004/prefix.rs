// Answer 0

#[test]
fn test_set_query_with_previous_query() {
    let mut url = Url::parse("https://example.com/products?page=1").unwrap();
    assert_eq!(url.as_str(), "https://example.com/products?page=1");
    
    url.set_query(None);
    assert_eq!(url.as_str(), "https://example.com/products");
    
    url.set_query(Some("page=2"));
    assert_eq!(url.as_str(), "https://example.com/products?page=2");
    assert_eq!(url.query(), Some("page=2"));
}

#[test]
fn test_set_query_with_valid_parameters() {
    let mut url = Url::parse("https://example.com/products?page=1").unwrap();
    
    url.set_query(None); // Clear previous query
    assert_eq!(url.as_str(), "https://example.com/products");
    
    url.set_query(Some("sort=asc&filter=active")); // Valid query
    assert_eq!(url.as_str(), "https://example.com/products?sort=asc&filter=active");
    assert_eq!(url.query(), Some("sort=asc&filter=active"));
}

#[test]
fn test_set_query_with_empty_strings() {
    let mut url = Url::parse("https://example.com/products?page=1").unwrap();
    
    url.set_query(None); // Clear previous query
    assert_eq!(url.as_str(), "https://example.com/products");
    
    url.set_query(Some("")); // Setting empty query
    assert_eq!(url.as_str(), "https://example.com/products?");
    assert_eq!(url.query(), Some(""));
}

