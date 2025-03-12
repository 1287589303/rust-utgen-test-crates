// Answer 0

#[test]
fn test_with_capacity_zero() {
    let capacity = 0;
    let map = Map::with_capacity(capacity);
}

#[test]
fn test_with_capacity_one() {
    let capacity = 1;
    let map = Map::with_capacity(capacity);
}

#[test]
fn test_with_capacity_positive() {
    let capacity = 10;
    let map = Map::with_capacity(capacity);
}

#[test]
fn test_with_capacity_max_usize() {
    let capacity = std::usize::MAX;
    let map = Map::with_capacity(capacity);
}

