// Answer 0

#[test]
fn test_utf8_enabled() {
    let config = Config::new().utf8(true);
}

#[test]
fn test_utf8_disabled() {
    let config = Config::new().utf8(false);
}

