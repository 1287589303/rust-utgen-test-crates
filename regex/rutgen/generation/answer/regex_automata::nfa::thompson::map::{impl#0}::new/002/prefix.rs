// Answer 0

#[test]
#[should_panic]
fn test_new_capacity_zero() {
    let _map = Utf8BoundedMap::new(0);
}

#[test]
fn test_new_capacity_one() {
    let _map = Utf8BoundedMap::new(1);
}

#[test]
fn test_new_capacity_ten() {
    let _map = Utf8BoundedMap::new(10);
}

#[test]
fn test_new_max_capacity() {
    let max_capacity = usize::MAX;
    let _map = Utf8BoundedMap::new(max_capacity);
}

