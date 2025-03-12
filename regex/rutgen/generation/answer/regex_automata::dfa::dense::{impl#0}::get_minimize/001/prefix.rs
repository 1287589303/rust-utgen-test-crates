// Answer 0

#[test]
fn test_get_minimize_some_true() {
    let config = Config::new().minimize(true);
    let result = config.get_minimize();
}

#[test]
fn test_get_minimize_some_false() {
    let config = Config::new().minimize(false);
    let result = config.get_minimize();
}

#[test]
fn test_get_minimize_none() {
    let config = Config::new();
    let result = config.get_minimize();
}

