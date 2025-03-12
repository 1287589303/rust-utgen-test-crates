// Answer 0

#[test]
fn test_capacity_zero() {
    let map: HashMap<i32, i32> = HashMap::with_capacity_and_hasher_in(0, DefaultHashBuilder, Global);
    let cap = map.capacity();
}

#[test]
fn test_capacity_one() {
    let map: HashMap<i32, i32> = HashMap::with_capacity_and_hasher_in(1, DefaultHashBuilder, Global);
    let cap = map.capacity();
}

#[test]
fn test_capacity_multiple() {
    let map: HashMap<i32, i32> = HashMap::with_capacity_and_hasher_in(10, DefaultHashBuilder, Global);
    let cap = map.capacity();
}

#[test]
fn test_capacity_max_usize() {
    let map: HashMap<i32, i32> = HashMap::with_capacity_and_hasher_in(usize::MAX, DefaultHashBuilder, Global);
    let cap = map.capacity();
}

