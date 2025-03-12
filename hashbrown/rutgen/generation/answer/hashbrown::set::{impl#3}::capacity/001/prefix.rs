// Answer 0

#[test]
fn test_capacity_zero() {
    let set: HashSet<i32> = HashSet::with_capacity(0);
    let _ = set.capacity();
}

#[test]
fn test_capacity_positive() {
    let set: HashSet<i32> = HashSet::with_capacity(1);
    let _ = set.capacity();
}

#[test]
fn test_capacity_large() {
    let set: HashSet<i32> = HashSet::with_capacity(1_000);
    let _ = set.capacity();
}

#[test]
fn test_capacity_boundary() {
    let set: HashSet<i32> = HashSet::with_capacity(usize::MAX);
    let _ = set.capacity();
}

