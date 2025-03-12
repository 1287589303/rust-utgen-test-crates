// Answer 0

#[test]
fn test_default_port_https() {
    let scheme = "https";
    let result = default_port(scheme);
}

#[test]
fn test_default_port_wss() {
    let scheme = "wss";
    let result = default_port(scheme);
}

