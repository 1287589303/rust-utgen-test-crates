// Answer 0

#[test]
fn test_insert_vacant_entry() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table: HashTable<&str> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);

    let entry = table
        .entry(hasher(&"test_key"), |&x| x == "test_key", hasher)
        .insert("test_value");

    let retrieved_value = entry.get();
}

#[test]
fn test_insert_vacant_entry_empty_table() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table: HashTable<i32> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);

    let entry = table
        .entry(hasher(&1), |&x| x == &1, hasher)
        .insert(100);

    let retrieved_value = entry.get();
}

#[test]
fn test_insert_vacant_entry_with_different_types() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table: HashTable<(i32, i32)> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);

    let entry = table
        .entry(hasher(&(1, 2)), |&x| x == &(1, 2), hasher)
        .insert((1, 2));

    let retrieved_value = entry.get();
}

