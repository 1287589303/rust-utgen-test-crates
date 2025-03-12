// Answer 0

#[test]
fn test_insert_existing_key() {
    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("key1", 10);
    if let OccupiedEntry { elem, .. } = map.entry("key1") {
        let old_value = elem.insert(20);
    }
}

#[test]
fn test_insert_non_existing_key() {
    let mut map: HashMap<&str, u32> = HashMap::new();
    if let OccupiedEntry { elem, .. } = map.entry("key2") {
        let old_value = elem.insert(30);
    }
}

#[test]
fn test_insert_with_multiple_entries() {
    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("key3", 5);
    map.insert("key4", 15);
    
    if let OccupiedEntry { elem, .. } = map.entry("key3") {
        let old_value = elem.insert(25);
    }
    
    if let OccupiedEntry { elem, .. } = map.entry("key4") {
        let old_value = elem.insert(35);
    }
}

#[test]
fn test_insert_boundary_conditions() {
    let mut map: HashMap<i32, i32> = HashMap::new();
    
    for i in 1..=100 {
        map.insert(i, i * 10);
    }
    
    if let OccupiedEntry { elem, .. } = map.entry(50) {
        let old_value = elem.insert(1000);
    }
    
    if let OccupiedEntry { elem, .. } = map.entry(101) {
        let old_value = elem.insert(2000);
    }
}

