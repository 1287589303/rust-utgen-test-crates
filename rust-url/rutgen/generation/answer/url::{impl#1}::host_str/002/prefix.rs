// Answer 0

#[test]
fn test_host_str_with_data_url() {
    let url = Url::parse("data:text/plain,Stuff").unwrap();
    let _ = url.host_str();
}

#[test]
fn test_host_str_with_mailto_url() {
    let url = Url::parse("mailto:email@example.com").unwrap();
    let _ = url.host_str();
}

#[test]
fn test_host_str_with_unix_socket_url() {
    let url = Url::parse("unix:/run/foo.socket").unwrap();
    let _ = url.host_str();
}

#[test]
fn test_host_str_with_empty_url() {
    let url = Url::parse(":/").unwrap();
    let _ = url.host_str();
}

#[test]
fn test_host_str_with_missing_scheme() {
    let url = Url::parse("example.com/path").unwrap();
    let _ = url.host_str();
}

#[test]
fn test_host_str_with_unqualified_url() {
    let url = Url::parse("path/to/resource").unwrap();
    let _ = url.host_str();
}

#[test]
fn test_host_str_with_special_file_url() {
    let url = Url::parse("file:///path/to/file").unwrap();
    let _ = url.host_str();
}

