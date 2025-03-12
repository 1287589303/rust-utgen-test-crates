// Answer 0

#[test]
fn test_get_cache_capacity_none() {
    let config = Config::new();
    let capacity = config.get_cache_capacity();
}

#[test]
fn test_get_cache_capacity_zero() {
    let config = Config::new().cache_capacity(0);
    let capacity = config.get_cache_capacity();
}

#[test]
fn test_get_cache_capacity_one() {
    let config = Config::new().cache_capacity(1);
    let capacity = config.get_cache_capacity();
}

#[test]
fn test_get_cache_capacity_default() {
    let config = Config::new().cache_capacity(2 * (1 << 20));
    let capacity = config.get_cache_capacity();
}

#[test]
fn test_get_cache_capacity_large_value() {
    let config = Config::new().cache_capacity(1048576);
    let capacity = config.get_cache_capacity();
}

