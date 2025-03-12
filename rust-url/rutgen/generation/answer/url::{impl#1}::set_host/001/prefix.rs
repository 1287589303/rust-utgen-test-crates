// Answer 0

#[test]
fn test_set_host_cannot_be_a_base_error_change_host() {
    let mut url = Url::parse("mailto:rms@example.net").unwrap();
    let result = url.set_host(Some("rust-lang.org"));
}

#[test]
fn test_set_host_cannot_be_a_base_error_remove_host() {
    let mut url = Url::parse("mailto:rms@example.net").unwrap();
    let result = url.set_host(None);
}

