// Answer 0

#[test]
fn test_or_insert_with_occupied_entry() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table: HashTable<&str> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);

    // Insert an initial value
    table.insert_unique(hasher(&"initial"), "initial", hasher);

    // Create an Entry::Occupied
    let entry = table.entry(hasher(&"initial"), |&x| x == "initial", hasher);
    entry.or_insert("new_value");
}

#[test]
fn test_or_insert_with_existing_key() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table: HashTable<&str> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);

    // Insert an initial value
    table.insert_unique(hasher(&"existing_key"), "existing_key", hasher);

    // Create an Entry::Occupied
    let entry = table.entry(hasher(&"existing_key"), |&x| x == "existing_key", hasher);
    let occupied_entry = entry.or_insert("new_value");

    // Test the return type of or_insert
    let _returned_entry: &str = occupied_entry.get(); // simulate usage of the returned entry
}

#[test]
fn test_or_insert_multiple_keys() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table: HashTable<&str> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);

    // Insert several unique values
    table.insert_unique(hasher(&"key1"), "value1", hasher);
    table.insert_unique(hasher(&"key2"), "value2", hasher);

    // Create an Entry::Occupied for key1
    let entry1 = table.entry(hasher(&"key1"), |&x| x == "value1", hasher);
    entry1.or_insert("new_value1");

    // Create an Entry::Occupied for key2
    let entry2 = table.entry(hasher(&"key2"), |&x| x == "value2", hasher);
    entry2.or_insert("new_value2");
}

