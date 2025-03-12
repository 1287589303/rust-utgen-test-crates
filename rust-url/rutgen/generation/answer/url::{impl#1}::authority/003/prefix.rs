// Answer 0

#[test]
fn test_authority_with_file_url() {
    let url = Url::parse("file:///tmp/foo").unwrap();
    let _ = url.authority();
}

#[test]
fn test_authority_with_unix_url() {
    let url = Url::parse("unix:/run/foo.socket").unwrap();
    let _ = url.authority();
}

#[test]
fn test_authority_with_invalid_url() {
    let url = Url::parse("://invalid").unwrap();
    let _ = url.authority();
}

#[test]
fn test_authority_with_no_authority_http_url() {
    let url = Url::parse("http://path/to/resource").unwrap();
    let _ = url.authority();
}

