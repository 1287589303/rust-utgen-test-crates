// Answer 0

#[test]
fn test_or_insert_vacant_entry_initial_empty() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table: HashTable<&str> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);
    
    let entry = table.entry(hasher(&"example"), |&x| x == "example", hasher);
    entry.or_insert("example");
}

#[test]
fn test_or_insert_vacant_entry_with_existing_entries() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table: HashTable<&str> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);
    
    table.insert(hasher(&"existing"), "existing");

    let entry = table.entry(hasher(&"example"), |&x| x == "example", hasher);
    entry.or_insert("example");
}

#[test]
fn test_or_insert_vacant_entry_with_different_hash() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table: HashTable<&str> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);
    
    table.insert(hasher(&"another"), "another");

    let entry = table.entry(hasher(&"unique"), |&x| x == "unique", hasher);
    entry.or_insert("unique");
}

#[test]
fn test_or_insert_multiple_vacant_entries() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table: HashTable<&str> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);
    
    let entry1 = table.entry(hasher(&"first"), |&x| x == "first", hasher);
    entry1.or_insert("first");

    let entry2 = table.entry(hasher(&"second"), |&x| x == "second", hasher);
    entry2.or_insert("second");
}

