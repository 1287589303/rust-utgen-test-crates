// Answer 0

#[test]
fn test_remove_ordered() {
    let mut map = serde_json::Map::new();
    map.insert("serde".to_owned(), Value::Number(12.into()));
    
    match map.entry("serde") {
        Entry::Occupied(occupied) => {
            occupied.remove();
        }
        Entry::Vacant(_) => unimplemented!(),
    }
}

#[test]
fn test_remove_unordered() {
    let mut map: BTreeMap<String, Value> = BTreeMap::new();
    map.insert("serde".to_owned(), Value::Number(12.into()));
    
    match map.entry("serde") {
        Entry::Occupied(occupied) => {
            occupied.remove();
        }
        Entry::Vacant(_) => unimplemented!(),
    }
}

