// Answer 0

#[test]
fn test_size_limit_some_zero() {
    let config = Config::new().size_limit(Some(0));
    let _ = config.get_size_limit();
}

#[test]
fn test_size_limit_some_one() {
    let config = Config::new().size_limit(Some(1));
    let _ = config.get_size_limit();
}

#[test]
fn test_size_limit_some_max() {
    let config = Config::new().size_limit(Some(usize::MAX));
    let _ = config.get_size_limit();
}

#[test]
fn test_size_limit_none() {
    let config = Config::new().size_limit(None);
    let _ = config.get_size_limit();
}

