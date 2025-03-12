// Answer 0

#[test]
fn test_drain_non_empty() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher_fn = |val: &_| hasher.hash_one(val);
    
    for x in 0..=3 {
        table.insert_unique(hasher_fn(&x), x, hasher_fn);
    }
    
    let drained: Vec<_> = table.drain().collect();
    
    assert!(!table.is_empty());
    assert_eq!(drained.len(), 4);
    assert!(table.is_empty());
}

#[test]
fn test_drain_empty() {
    use hashbrown::HashTable;

    let mut table: HashTable<i32> = HashTable::new();
    let drained: Vec<_> = table.drain().collect();
    
    assert_eq!(drained.len(), 0);
    assert!(table.is_empty());
}

