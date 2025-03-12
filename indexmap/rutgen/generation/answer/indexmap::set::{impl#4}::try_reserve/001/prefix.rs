// Answer 0

#[test]
fn test_try_reserve_zero() {
    let mut set: super::IndexSet<usize, ()> = super::IndexSet::with_capacity_and_hasher(0, ());
    let result = set.try_reserve(0);
}

#[test]
fn test_try_reserve_one() {
    let mut set: super::IndexSet<usize, ()> = super::IndexSet::with_capacity_and_hasher(0, ());
    let result = set.try_reserve(1);
}

#[test]
fn test_try_reserve_max_usize() {
    let mut set: super::IndexSet<usize, ()> = super::IndexSet::with_capacity_and_hasher(0, ());
    let result = set.try_reserve(usize::MAX);
}

