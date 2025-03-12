// Answer 0

#[test]
fn test_set_scheme_to_special_valid() {
    let mut url = Url::parse("http://")?;
    url.set_scheme("https").unwrap();
}

#[test]
fn test_set_scheme_to_special_valid_user_defined() {
    let mut url = Url::parse("http://")?;
    url.set_scheme("myprotocol").unwrap();
}

#[test]
fn test_set_scheme_to_special_non_special() {
    let mut url = Url::parse("file://")?;
    url.set_scheme("myprotocol").unwrap();
}

#[test]
fn test_set_scheme_to_special_empty_host() {
    let mut url = Url::parse("myprotocol://")?;
    url.set_scheme("http").unwrap();
}

#[test]
fn test_set_scheme_to_special_with_query() {
    let mut url = Url::parse("http://example.com?query=1")?;
    url.set_scheme("https").unwrap();
}

#[test]
fn test_set_scheme_to_special_with_fragment() {
    let mut url = Url::parse("http://example.com#fragment")?;
    url.set_scheme("https").unwrap();
}

