// Answer 0

#[test]
fn test_key_on_vacant_entry() {
    use hashbrown::hash_map::{Entry, HashMap};
    
    let mut map: HashMap<&str, u32> = HashMap::new();
    let key = "horseland";
    let entry = map.entry(key);
    let _ = entry.key(); // Calling key on a Vacant entry
}

#[test]
fn test_key_on_occupied_entry() {
    use hashbrown::hash_map::{Entry, HashMap};
    
    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("poneyland", 3);
    let entry = map.entry("poneyland");
    let _ = entry.key(); // Calling key on an Occupied entry
}

