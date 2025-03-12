// Answer 0

#[test]
fn test_reserve_zero() {
    let mut set: HashSet<i32> = HashSet::new();
    set.reserve(0);
}

#[test]
fn test_reserve_small() {
    let mut set: HashSet<i32> = HashSet::new();
    set.reserve(5);
}

#[test]
fn test_reserve_large() {
    let mut set: HashSet<i32> = HashSet::new();
    set.reserve(isize::MAX as usize);
}

#[should_panic]
fn test_reserve_exceeds_capacity() {
    let mut set: HashSet<i32> = HashSet::new();
    set.reserve(isize::MAX as usize + 1);
}

