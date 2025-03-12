// Answer 0

#[test]
fn test_cannot_be_a_base_valid_urls() {
    let url = Url::parse("http://example.com").unwrap();
    url.cannot_be_a_base();

    let url = Url::parse("ftp://example.com").unwrap();
    url.cannot_be_a_base();

    let url = Url::parse("unix:/run/foo.socket").unwrap();
    url.cannot_be_a_base();
}

#[test]
fn test_cannot_be_a_base_non_hierarchical_urls() {
    let url = Url::parse("data:text/plain,Stuff").unwrap();
    url.cannot_be_a_base();

    let url = Url::parse("mailto:someone@example.com").unwrap();
    url.cannot_be_a_base();
}

