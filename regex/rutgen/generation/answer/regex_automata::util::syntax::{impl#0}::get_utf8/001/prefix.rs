// Answer 0

#[test]
fn test_get_utf8_true() {
    let config = Config::new().utf8(true);
    let result = config.get_utf8();
}

#[test]
fn test_get_utf8_false() {
    let config = Config::new().utf8(false);
    let result = config.get_utf8();
}

