// Answer 0

#[test]
fn test_with_capacity_and_hasher_zero_capacity() {
    use hashbrown::{HashMap, DefaultHashBuilder};

    let s = DefaultHashBuilder::default();
    let map = HashMap::with_capacity_and_hasher(0, s);
}

#[test]
fn test_with_capacity_and_hasher_small_capacity() {
    use hashbrown::{HashMap, DefaultHashBuilder};

    let s = DefaultHashBuilder::default();
    let map = HashMap::with_capacity_and_hasher(1, s);
}

#[test]
fn test_with_capacity_and_hasher_large_capacity() {
    use hashbrown::{HashMap, DefaultHashBuilder};

    let s = DefaultHashBuilder::default();
    let map = HashMap::with_capacity_and_hasher(100, s);
}

#[test]
fn test_with_capacity_and_hasher_max_capacity() {
    use hashbrown::{HashMap, DefaultHashBuilder};

    let s = DefaultHashBuilder::default();
    let map = HashMap::with_capacity_and_hasher(usize::MAX, s);
}

