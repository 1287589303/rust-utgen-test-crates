// Answer 0

#[test]
fn test_entry_debug_occupied() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table: HashTable<&str, &str, DefaultHashBuilder> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    
    table.insert("key1", "value1");
    
    let occupied_entry = match table.entry("key1", |&x| x == "key1", hasher) {
        Entry::Occupied(o) => o,
        _ => panic!("Expected occupied entry"),
    };

    let _ = occupied_entry.fmt(&mut core::fmt::Formatter::new());
}

