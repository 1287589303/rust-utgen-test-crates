// Answer 0

#[test]
fn test_or_insert_with_key_vacant_entry() {
    struct MyHashBuilder;

    let mut map: HashMap<&str, usize> = HashMap::new();
    let entry = map.entry("unique_key");
    
    entry.or_insert_with_key(|key| key.chars().count());
}

#[test]
fn test_or_insert_with_key_vacant_entry_empty_function() {
    struct MyHashBuilder;

    let mut map: HashMap<&str, usize> = HashMap::new();
    let entry = map.entry("another_unique_key");
    
    entry.or_insert_with_key(|_key| 42);
}

#[test]
fn test_or_insert_with_key_vacant_entry_multiple_calls() {
    struct MyHashBuilder;

    let mut map: HashMap<&str, usize> = HashMap::new();
    let entry1 = map.entry("first_key");
    
    entry1.or_insert_with_key(|key| key.len());
    
    let entry2 = map.entry("second_key");
    
    entry2.or_insert_with_key(|key| key.chars().count());
}

#[test]
fn test_or_insert_with_key_vacant_entry_same_key() {
    struct MyHashBuilder;

    let mut map: HashMap<&str, usize> = HashMap::new();
    let entry = map.entry("duplicate_key");
    
    entry.or_insert_with_key(|key| key.len());
    entry.or_insert_with_key(|key| key.chars().count());
}

#[test]
fn test_or_insert_with_key_vacant_entry_different_values() {
    struct MyHashBuilder;

    let mut map: HashMap<&str, usize> = HashMap::new();
    let entry = map.entry("key_with_value");
    
    entry.or_insert_with_key(|key| key.chars().count());
    let value = *entry.or_insert_with_key(|key| key.len());
}

