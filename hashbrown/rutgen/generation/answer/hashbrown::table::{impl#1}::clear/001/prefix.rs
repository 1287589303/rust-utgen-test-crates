// Answer 0

#[test]
fn test_clear_empty_table() {
    let mut table: HashTable<i32> = HashTable::new_in(Global);
    table.clear();
    table.is_empty();
}

#[test]
fn test_clear_single_insert() {
    let mut table: HashTable<i32> = HashTable::new_in(Global);
    let hasher = |val: &i32| *val as u64; // simple hasher
    table.insert_unique(hasher(&1), 1, hasher);
    table.clear();
    table.is_empty();
}

#[test]
fn test_clear_multiple_inserts() {
    let mut table: HashTable<i32> = HashTable::new_in(Global);
    let hasher = |val: &i32| *val as u64; // simple hasher
    for i in 1..=5 {
        table.insert_unique(hasher(&i), i, hasher);
    }
    table.clear();
    table.is_empty();
}

#[test]
fn test_clear_after_clear() {
    let mut table: HashTable<i32> = HashTable::new_in(Global);
    table.clear(); // first clear on empty table
    table.clear(); // second clear on already empty table
    table.is_empty();
}

#[test]
fn test_clear_with_string_inserts() {
    let mut table: HashTable<String> = HashTable::new_in(Global);
    let hasher = |val: &String| {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        val.hash(&mut hasher);
        hasher.finish()
    }; 
    let values = vec!["one".to_string(), "two".to_string(), "three".to_string()];
    for value in &values {
        table.insert_unique(hasher(value), value.clone(), hasher);
    }
    table.clear();
    table.is_empty();
}

