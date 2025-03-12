// Answer 0

#[test]
fn test_reserve_zero_additional() {
    let mut table: HashTable<i32> = HashTable::new_in(Global);
    let hasher = |val: &i32| *val as u64;
    table.reserve(0, hasher);
}

#[test]
fn test_reserve_small_additional() {
    let mut table: HashTable<i32> = HashTable::new_in(Global);
    let hasher = |val: &i32| *val as u64;
    table.reserve(5, hasher);
}

#[test]
fn test_reserve_large_additional() {
    let mut table: HashTable<i32> = HashTable::new_in(Global);
    let hasher = |val: &i32| *val as u64;
    table.reserve(usize::MAX - 1, hasher);
}

#[should_panic]
fn test_reserve_exceed_isize_max() {
    let mut table: HashTable<i32> = HashTable::new_in(Global);
    let hasher = |val: &i32| *val as u64;
    table.reserve(isize::MAX as usize + 1, hasher);
}

