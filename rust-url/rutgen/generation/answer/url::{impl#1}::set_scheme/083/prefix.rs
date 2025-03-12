// Answer 0

#[test]
fn test_set_scheme_http_to_ftp() {
    let mut url = Url::parse("ftp://example.net").unwrap();
    let result = url.set_scheme("http");
}

#[test]
fn test_set_scheme_ftp_to_ftp() {
    let mut url = Url::parse("ftp://example.net").unwrap();
    let result = url.set_scheme("ftp");
}

#[test]
fn test_set_scheme_ftp_to_file() {
    let mut url = Url::parse("ftp://example.net").unwrap();
    let result = url.set_scheme("file");
}

