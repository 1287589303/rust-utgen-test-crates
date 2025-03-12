// Answer 0

#[test]
fn test_entry_ref_occupied_debug() {
    use hashbrown::hash_map::{EntryRef, HashMap};

    let mut map: HashMap<String, i32> = HashMap::new();
    map.insert("key1".to_owned(), 42);

    let occupied_entry = map.entry_ref("key1");
    
    if let EntryRef::Occupied(ref occupied) = occupied_entry {
        let _ = fmt::Debug::fmt(&occupied, &mut fmt::Formatter::new());
    }
}

#[test]
fn test_entry_ref_occupied_debug_with_different_key_value() {
    use hashbrown::hash_map::{EntryRef, HashMap};

    let mut map: HashMap<i32, String> = HashMap::new();
    map.insert(1, "value1".to_owned());

    let occupied_entry = map.entry_ref(&1);
    
    if let EntryRef::Occupied(ref occupied) = occupied_entry {
        let _ = fmt::Debug::fmt(&occupied, &mut fmt::Formatter::new());
    }
}

#[test]
fn test_entry_ref_occupied_debug_with_multiple_entries() {
    use hashbrown::hash_map::{EntryRef, HashMap};

    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("a", 100);
    map.insert("b", 200);

    let occupied_entry = map.entry_ref("a");
    
    if let EntryRef::Occupied(ref occupied) = occupied_entry {
        let _ = fmt::Debug::fmt(&occupied, &mut fmt::Formatter::new());
    }
}

