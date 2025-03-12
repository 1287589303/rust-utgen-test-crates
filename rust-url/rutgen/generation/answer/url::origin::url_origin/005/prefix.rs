// Answer 0

#[test]
fn test_url_origin_https_scheme() {
    let input_url = "https://example.com:443/path";
    let url = Url::parse(input_url).unwrap();
    let origin = url_origin(&url);
}

#[test]
fn test_url_origin_https_scheme_no_port() {
    let input_url = "https://example.com/path";
    let url = Url::parse(input_url).unwrap();
    let origin = url_origin(&url);
}

#[test]
fn test_url_origin_https_scheme_with_subdomain() {
    let input_url = "https://sub.example.com:443/path";
    let url = Url::parse(input_url).unwrap();
    let origin = url_origin(&url);
}

#[test]
fn test_url_origin_https_scheme_ipv4() {
    let input_url = "https://192.168.1.1:443/path";
    let url = Url::parse(input_url).unwrap();
    let origin = url_origin(&url);
}

#[test]
fn test_url_origin_https_scheme_ipv6() {
    let input_url = "https://[2001:db8::1]:443/path";
    let url = Url::parse(input_url).unwrap();
    let origin = url_origin(&url);
}

