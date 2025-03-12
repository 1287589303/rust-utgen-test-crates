// Answer 0

#[test]
fn test_url_has_authority_with_authority() {
    let url = Url::parse("http://user:password@host.com:8080/path").unwrap();
    let _ = url.has_authority();
}

#[test]
fn test_url_has_authority_with_userinfo() {
    let url = Url::parse("ftp://rms@example.com").unwrap();
    let _ = url.has_authority();
}

#[test]
fn test_url_has_no_authority_with_path_only() {
    let url = Url::parse("unix:/run/foo.socket").unwrap();
    let _ = url.has_authority();
}

#[test]
fn test_url_has_no_authority_with_cannot_be_base() {
    let url = Url::parse("data:text/plain,Stuff").unwrap();
    let _ = url.has_authority();
}

