// Answer 0

#[test]
fn test_capacity_zero() {
    let table: HashTable<i32> = HashTable::with_capacity_in(0, Global);
    let _cap = table.capacity();
}

#[test]
fn test_capacity_small() {
    let table: HashTable<i32> = HashTable::with_capacity_in(1, Global);
    let _cap = table.capacity();
}

#[test]
fn test_capacity_small_multiple() {
    let table: HashTable<i32> = HashTable::with_capacity_in(5, Global);
    let _cap = table.capacity();
}

#[test]
fn test_capacity_large() {
    let table: HashTable<i32> = HashTable::with_capacity_in(1024, Global);
    let _cap = table.capacity();
}

#[test]
fn test_capacity_max() {
    let table: HashTable<i32> = HashTable::with_capacity_in(1 << 30, Global);
    let _cap = table.capacity();
}

#[test]
fn test_capacity_exceeds_max() {
    let table: HashTable<i32> = HashTable::with_capacity_in((1 << 30) + 1, Global);
    let _cap = table.capacity();
}

