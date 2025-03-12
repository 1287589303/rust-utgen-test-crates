// Answer 0

#[test]
fn test_get_shrink_none() {
    let config = Config::default();
    let result = config.get_shrink();
}

#[test]
fn test_get_shrink_true() {
    let config = Config::default().shrink(true);
    let result = config.get_shrink();
}

#[test]
fn test_get_shrink_false() {
    let config = Config::default().shrink(false);
    let result = config.get_shrink();
}

