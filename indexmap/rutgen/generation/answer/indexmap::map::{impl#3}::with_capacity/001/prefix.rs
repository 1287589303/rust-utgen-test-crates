// Answer 0

#[test]
fn test_with_capacity_zero() {
    let map: IndexMap<usize, usize> = IndexMap::with_capacity(0);
}

#[test]
fn test_with_capacity_one() {
    let map: IndexMap<usize, usize> = IndexMap::with_capacity(1);
}

#[test]
fn test_with_capacity_ten() {
    let map: IndexMap<usize, usize> = IndexMap::with_capacity(10);
}

#[test]
fn test_with_capacity_max() {
    let map: IndexMap<usize, usize> = IndexMap::with_capacity(usize::MAX);
}

