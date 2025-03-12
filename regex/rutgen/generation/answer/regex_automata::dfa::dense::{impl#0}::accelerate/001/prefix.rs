// Answer 0

#[test]
fn test_accelerate_true() {
    let config = Config::new();
    let updated_config = config.accelerate(true);
}

#[test]
fn test_accelerate_false() {
    let config = Config::new();
    let updated_config = config.accelerate(false);
}

