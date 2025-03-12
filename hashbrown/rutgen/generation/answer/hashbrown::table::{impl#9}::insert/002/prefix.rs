// Answer 0

#[test]
fn test_insert_with_occupied_entry() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table: HashTable<&str> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher_fn = |val: &_| hasher.hash_one(val);

    table.insert_unique(hasher_fn(&"key1"), "value1", hasher_fn);
    let entry = table.entry(hasher_fn(&"key1"), |&x| x == "key1", hasher_fn);

    let occupied_entry = entry.insert("new_value");

    // Calling this function for side effects: testing the method.
    let _result = occupied_entry.get();
}

#[test]
fn test_insert_with_another_occupied_entry() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table: HashTable<&str> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher_fn = |val: &_| hasher.hash_one(val);

    table.insert_unique(hasher_fn(&"key2"), "old_value", hasher_fn);
    let entry = table.entry(hasher_fn(&"key2"), |&x| x == "key2", hasher_fn);

    let occupied_entry = entry.insert("updated_value");

    // Calling this function for side effects: testing the method.
    let _result = occupied_entry.get();
}

