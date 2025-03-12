// Answer 0

#[test]
fn test_get_onepass_size_limit_none() {
    let config = Config::new();
    let _ = config.get_onepass_size_limit();
}

#[test]
fn test_get_onepass_size_limit_zero() {
    let config = Config::new().onepass_size_limit(Some(0));
    let _ = config.get_onepass_size_limit();
}

#[test]
fn test_get_onepass_size_limit_one() {
    let config = Config::new().onepass_size_limit(Some(1));
    let _ = config.get_onepass_size_limit();
}

#[test]
fn test_get_onepass_size_limit_max() {
    let config = Config::new().onepass_size_limit(Some(2097152));
    let _ = config.get_onepass_size_limit();
}

