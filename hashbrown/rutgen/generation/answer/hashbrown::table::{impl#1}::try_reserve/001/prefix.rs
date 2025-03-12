// Answer 0

#[test]
fn test_try_reserve_zero_additional() {
    let mut table: HashTable<i32> = HashTable::new_in(Global);
    let hasher = |val: &i32| *val as u64;
    table.try_reserve(0, hasher).unwrap();
}

#[test]
fn test_try_reserve_small_additional() {
    let mut table: HashTable<i32> = HashTable::new_in(Global);
    let hasher = |val: &i32| *val as u64;
    table.try_reserve(5, hasher).unwrap();
}

#[test]
#[should_panic]
fn test_try_reserve_overflow() {
    let mut table: HashTable<i32> = HashTable::new_in(Global);
    let hasher = |val: &i32| *val as u64;
    table.try_reserve(isize::MAX as usize, hasher).unwrap();
}

#[test]
fn test_try_reserve_large_additional() {
    let mut table: HashTable<i32> = HashTable::with_capacity_in(100, Global);
    let hasher = |val: &i32| *val as u64;
    table.try_reserve(1000, hasher).unwrap();
}

