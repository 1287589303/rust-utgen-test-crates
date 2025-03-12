// Answer 0

#[test]
fn test_onepass_true() {
    let config = Config::new();
    let modified_config = config.onepass(true);
}

#[test]
fn test_onepass_false() {
    let config = Config::new();
    let modified_config = config.onepass(false);
}

