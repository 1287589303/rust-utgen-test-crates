// Answer 0

#[test]
fn test_get_skip_cache_capacity_check_none() {
    let config = Config::default();
    let result = config.get_skip_cache_capacity_check();
}

#[test]
fn test_get_skip_cache_capacity_check_false() {
    let config = Config::default().skip_cache_capacity_check(false);
    let result = config.get_skip_cache_capacity_check();
}

#[test]
fn test_get_skip_cache_capacity_check_true() {
    let config = Config::default().skip_cache_capacity_check(true);
    let result = config.get_skip_cache_capacity_check();
}

