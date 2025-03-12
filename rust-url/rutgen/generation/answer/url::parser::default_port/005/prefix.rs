// Answer 0

#[test]
fn test_default_port_ftp() {
    let scheme = "ftp";
    let result = url::default_port(scheme);
}

#[test]
fn test_default_port_invalid_scheme() {
    let scheme = "xyz";
    let result = url::default_port(scheme);
}

#[test]
fn test_default_port_http() {
    let scheme = "http";
    let result = url::default_port(scheme);
}

#[test]
fn test_default_port_https() {
    let scheme = "https";
    let result = url::default_port(scheme);
}

#[test]
fn test_default_port_ws() {
    let scheme = "ws";
    let result = url::default_port(scheme);
}

#[test]
fn test_default_port_wss() {
    let scheme = "wss";
    let result = url::default_port(scheme);
}

