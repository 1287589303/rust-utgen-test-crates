// Answer 0

#[test]
fn test_host_none_with_data_url() {
    let url = Url::parse("data:text/plain,Stuff").unwrap();
    let host = url.host();
}

#[test]
fn test_host_none_with_unix_socket_url() {
    let url = Url::parse("unix:/run/foo.socket").unwrap();
    let host = url.host();
}

