// Answer 0

#[test]
fn test_cache_capacity_zero() {
    let config = Config::new().cache_capacity(0);
}

#[test]
fn test_cache_capacity_one_mb() {
    let config = Config::new().cache_capacity(1 * (1 << 20));
}

#[test]
fn test_cache_capacity_ten_mb() {
    let config = Config::new().cache_capacity(10 * (1 << 20));
}

#[test]
fn test_cache_capacity_hundred_mb() {
    let config = Config::new().cache_capacity(100 * (1 << 20));
}

#[test]
fn test_cache_capacity_one_gb() {
    let config = Config::new().cache_capacity(1 * (1 << 30));
}

