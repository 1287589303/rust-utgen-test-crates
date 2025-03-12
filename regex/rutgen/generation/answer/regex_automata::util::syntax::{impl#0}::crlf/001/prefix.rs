// Answer 0

#[test]
fn test_crlf_enabled() {
    let config = Config::new().crlf(true);
}

#[test]
fn test_crlf_disabled() {
    let config = Config::new().crlf(false);
}

