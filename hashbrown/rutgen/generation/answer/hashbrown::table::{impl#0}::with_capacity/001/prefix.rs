// Answer 0

#[test]
fn test_with_capacity_zero() {
    let table: HashTable<&str> = HashTable::with_capacity(0);
}

#[test]
fn test_with_capacity_one() {
    let table: HashTable<&str> = HashTable::with_capacity(1);
}

#[test]
fn test_with_capacity_ten() {
    let table: HashTable<&str> = HashTable::with_capacity(10);
}

#[test]
fn test_with_capacity_one_hundred() {
    let table: HashTable<&str> = HashTable::with_capacity(100);
}

#[test]
fn test_with_capacity_max_usize() {
    let table: HashTable<&str> = HashTable::with_capacity(usize::MAX);
}

