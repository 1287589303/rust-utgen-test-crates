// Answer 0

#[test]
fn test_fmt_valid_http_url() {
    let url = Url::parse("http://user:pass@host.com:80/path?query#fragment").unwrap();
    let _ = format!("{:?}", url);
}

#[test]
fn test_fmt_valid_https_url() {
    let url = Url::parse("https://host.com").unwrap();
    let _ = format!("{:?}", url);
}

#[test]
fn test_fmt_valid_ftp_url() {
    let url = Url::parse("ftp://user@192.168.1.1:22/path").unwrap();
    let _ = format!("{:?}", url);
}

#[test]
fn test_fmt_valid_ipv6_url() {
    let url = Url::parse("http://[::1]").unwrap();
    let _ = format!("{:?}", url);
}

#[test]
fn test_fmt_valid_file_url() {
    let url = Url::parse("file:///path/to/file").unwrap();
    let _ = format!("{:?}", url);
}

#[test]
fn test_fmt_empty_url() {
    let url = Url::parse("").unwrap_err();
    let _ = format!("{:?}", url);
}

#[test]
fn test_fmt_excessive_components_url() {
    let url = Url::parse("http://user:pass@host.com:80/path/to/component/with/too/many/segments").unwrap();
    let _ = format!("{:?}", url);
}

