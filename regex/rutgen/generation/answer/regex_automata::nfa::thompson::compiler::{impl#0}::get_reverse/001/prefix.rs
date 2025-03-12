// Answer 0

#[test]
fn test_get_reverse_some_true() {
    let config = Config::new().reverse(true);
    let result = config.get_reverse();
}

#[test]
fn test_get_reverse_some_false() {
    let config = Config::new().reverse(false);
    let result = config.get_reverse();
}

#[test]
fn test_get_reverse_none() {
    let config = Config::new();
    let result = config.get_reverse();
}

