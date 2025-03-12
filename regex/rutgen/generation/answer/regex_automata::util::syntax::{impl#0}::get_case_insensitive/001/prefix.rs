// Answer 0

#[test]
fn test_get_case_insensitive_true() {
    let config = Config::new().case_insensitive(true);
    let result = config.get_case_insensitive();
}

#[test]
fn test_get_case_insensitive_false() {
    let config = Config::new().case_insensitive(false);
    let result = config.get_case_insensitive();
}

