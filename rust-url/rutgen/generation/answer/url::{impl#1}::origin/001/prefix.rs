// Answer 0

#[test]
fn test_origin_ftp() {
    let url = Url::parse("ftp://example.com/foo").unwrap();
    let _ = url.origin();
}

#[test]
fn test_origin_blob() {
    let url = Url::parse("blob:https://example.com/foo").unwrap();
    let _ = url.origin();
}

#[test]
fn test_origin_file() {
    let url = Url::parse("file:///tmp/foo").unwrap();
    let _ = url.origin();

    let other_url = Url::parse("file:///tmp/foo").unwrap();
    let _ = url.origin();
    let _ = other_url.origin();
}

#[test]
fn test_origin_other_scheme() {
    let url = Url::parse("foo:bar").unwrap();
    let _ = url.origin();
}

#[test]
fn test_origin_edge_case_missing_scheme() {
    let url = Url::parse("://invalid").unwrap();
    let _ = url.origin();
}

#[test]
fn test_origin_custom_scheme() {
    let url = Url::parse("custom_scheme://resource").unwrap();
    let _ = url.origin();
}

