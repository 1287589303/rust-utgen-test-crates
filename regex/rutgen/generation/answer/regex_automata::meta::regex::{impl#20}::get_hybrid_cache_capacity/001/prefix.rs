// Answer 0

#[test]
fn test_get_hybrid_cache_capacity_with_zero() {
    let config = Config::new().hybrid_cache_capacity(0);
    let _ = config.get_hybrid_cache_capacity();
}

#[test]
fn test_get_hybrid_cache_capacity_with_small_value() {
    let config = Config::new().hybrid_cache_capacity(1);
    let _ = config.get_hybrid_cache_capacity();
}

#[test]
fn test_get_hybrid_cache_capacity_with_large_value() {
    let config = Config::new().hybrid_cache_capacity(usize::MAX);
    let _ = config.get_hybrid_cache_capacity();
}

#[test]
fn test_get_hybrid_cache_capacity_with_unset() {
    let config = Config::new(); // hybrid_cache_capacity is not set
    let _ = config.get_hybrid_cache_capacity();
}

