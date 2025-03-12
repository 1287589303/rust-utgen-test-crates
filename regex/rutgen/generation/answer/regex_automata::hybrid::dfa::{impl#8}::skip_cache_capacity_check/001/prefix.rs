// Answer 0

#[test]
fn test_skip_cache_capacity_check_true() {
    let config = Config::new().skip_cache_capacity_check(true);
}

#[test]
fn test_skip_cache_capacity_check_false() {
    let config = Config::new().skip_cache_capacity_check(false);
}

