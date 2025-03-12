// Answer 0

#[test]
fn test_hash_table_new_empty() {
    let table: HashTable<&str> = HashTable::new();
    let length = table.raw.table.len(); // Length should be 0
    let capacity = table.raw.table.capacity(); // Capacity should be 0
}

#[test]
fn test_hash_table_new_zero_capacity() {
    let table: HashTable<&str> = HashTable::with_capacity(0);
    let length = table.raw.table.len(); // Length should be 0
    let capacity = table.raw.table.capacity(); // Capacity should be 0
}

