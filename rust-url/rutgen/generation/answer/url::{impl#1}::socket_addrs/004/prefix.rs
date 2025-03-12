// Answer 0

#[test]
fn test_socket_addrs_ipv4() {
    let url = url::Url::parse("http://192.168.1.1:8080").unwrap();
    let addrs = url.socket_addrs(|| Some(80)).unwrap();
}

#[test]
fn test_socket_addrs_ipv4_default_port() {
    let url = url::Url::parse("http://10.0.0.1").unwrap();
    let addrs = url.socket_addrs(|| Some(8080)).unwrap();
}

#[test]
fn test_socket_addrs_ipv4_no_port() {
    let url = url::Url::parse("http://172.16.0.1:3000").unwrap();
    let addrs = url.socket_addrs(|| None).unwrap();
}

