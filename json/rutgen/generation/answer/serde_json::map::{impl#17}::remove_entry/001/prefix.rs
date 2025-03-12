// Answer 0

#[test]
fn test_remove_entry_with_preserve_order() {
    use serde_json::json;
    use serde_json::map::Map;
    use serde_json::map::Entry;
    
    let mut map = Map::new();
    map.insert("key1".to_owned(), json!(12));
    map.insert("key2".to_owned(), json!(true));
    
    match map.entry("key1") {
        Entry::Occupied(occupied) => {
            let (key, value) = occupied.remove_entry();
            // Function is called, but assertions are omitted as per instructions
        }
        Entry::Vacant(_) => unimplemented!(),
    }
}

#[test]
fn test_remove_entry_without_preserve_order() {
    use serde_json::json;
    use serde_json::map::Map;
    use serde_json::map::Entry;
    
    let mut map = Map::new();
    map.insert("key1".to_owned(), json!(12.5));
    map.insert("key2".to_owned(), json!("value"));
    
    match map.entry("key2") {
        Entry::Occupied(occupied) => {
            let (key, value) = occupied.remove_entry();
            // Function is called, but assertions are omitted as per instructions
        }
        Entry::Vacant(_) => unimplemented!(),
    }
}

