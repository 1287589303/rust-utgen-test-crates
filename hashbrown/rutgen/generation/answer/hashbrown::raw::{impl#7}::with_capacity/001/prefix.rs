// Answer 0

#[test]
fn test_with_capacity_zero() {
    let table: RawTable<i32> = RawTable::with_capacity(0);
}

#[test]
fn test_with_capacity_one() {
    let table: RawTable<i32> = RawTable::with_capacity(1);
}

#[test]
fn test_with_capacity_large() {
    let table: RawTable<i32> = RawTable::with_capacity(usize::MAX);
}

