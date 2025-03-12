// Answer 0

#[test]
fn test_get_visited_capacity_none() {
    let config = Config::new();
    let capacity = config.get_visited_capacity();
}

#[test]
fn test_get_visited_capacity_zero() {
    let config = Config::new().visited_capacity(0);
    let capacity = config.get_visited_capacity();
}

#[test]
fn test_get_visited_capacity_one() {
    let config = Config::new().visited_capacity(1);
    let capacity = config.get_visited_capacity();
}

#[test]
fn test_get_visited_capacity_256() {
    let config = Config::new().visited_capacity(256);
    let capacity = config.get_visited_capacity();
}

#[test]
fn test_get_visited_capacity_256_kb() {
    let config = Config::new().visited_capacity(256 * (1 << 10));
    let capacity = config.get_visited_capacity();
}

#[test]
fn test_get_visited_capacity_500_kb() {
    let config = Config::new().visited_capacity(500 * (1 << 10));
    let capacity = config.get_visited_capacity();
}

