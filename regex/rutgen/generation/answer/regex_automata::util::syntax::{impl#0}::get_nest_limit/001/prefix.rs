// Answer 0

#[test]
fn test_get_nest_limit_zero() {
    let config = Config::new().nest_limit(0);
    let _ = config.get_nest_limit();
}

#[test]
fn test_get_nest_limit_one() {
    let config = Config::new().nest_limit(1);
    let _ = config.get_nest_limit();
}

#[test]
fn test_get_nest_limit_max() {
    let config = Config::new().nest_limit(u32::MAX);
    let _ = config.get_nest_limit();
}

