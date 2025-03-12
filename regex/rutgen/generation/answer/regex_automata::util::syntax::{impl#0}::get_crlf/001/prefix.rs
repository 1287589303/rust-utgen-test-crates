// Answer 0

#[test]
fn test_get_crlf_true() {
    let config = Config::new().crlf(true);
    let result = config.get_crlf();
}

#[test]
fn test_get_crlf_false() {
    let config = Config::new().crlf(false);
    let result = config.get_crlf();
}

