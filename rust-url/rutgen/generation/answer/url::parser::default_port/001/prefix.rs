// Answer 0

#[test]
fn test_default_port_http() {
    let scheme = "http";
    let _result = default_port(scheme);
}

#[test]
fn test_default_port_ws() {
    let scheme = "ws";
    let _result = default_port(scheme);
}

