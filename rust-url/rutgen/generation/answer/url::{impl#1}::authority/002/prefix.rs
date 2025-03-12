// Answer 0

#[test]
fn test_authority_empty() {
    let input = "http://example.com";
    let url = Url::parse(input).unwrap();
    let path_start = url.scheme_end + "://".len() as u32;
    url.path_start = path_start; // Adjust path_start to be equal to scheme_end + "://".len()
    assert_eq!(url.authority(), "");
}

#[test]
fn test_authority_empty_with_username_password() {
    let input = "https://user:password@example.com";
    let url = Url::parse(input).unwrap();
    let path_start = url.scheme_end + "://".len() as u32;
    url.path_start = path_start; // Adjust path_start
    assert_eq!(url.authority(), "");
}

#[test]
fn test_authority_empty_special_url() {
    let input = "file:///tmp/foo";
    let url = Url::parse(input).unwrap();
    let path_start = url.scheme_end + "://".len() as u32;
    url.path_start = path_start; // Adjust path_start
    assert_eq!(url.authority(), "");
}

