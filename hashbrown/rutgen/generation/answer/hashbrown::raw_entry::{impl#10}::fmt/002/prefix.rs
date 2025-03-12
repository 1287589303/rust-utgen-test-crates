// Answer 0

#[test]
fn test_raw_entry_occupied_fmt() {
    use hashbrown::{hash_map::RawEntryMut, HashMap};

    let mut map: HashMap<&str, i32> = HashMap::new();
    map.insert("key1", 1);
    
    let entry = match map.raw_entry_mut().from_key(&"key1") {
        RawEntryMut::Occupied(ref occupied) => occupied,
        RawEntryMut::Vacant(_) => panic!("Expected occupied entry"),
    };

    let mut formatter = fmt::Formatter::new();
    let _ = entry.fmt(&mut formatter);
}

#[test]
fn test_raw_entry_occupied_fmt_another_entry() {
    use hashbrown::{hash_map::RawEntryMut, HashMap};

    let mut map: HashMap<&str, i32> = HashMap::new();
    map.insert("key2", 2);

    let entry = match map.raw_entry_mut().from_key(&"key2") {
        RawEntryMut::Occupied(ref occupied) => occupied,
        RawEntryMut::Vacant(_) => panic!("Expected occupied entry"),
    };

    let mut formatter = fmt::Formatter::new();
    let _ = entry.fmt(&mut formatter);
}

