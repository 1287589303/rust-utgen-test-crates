// Answer 0

#[test]
fn test_key_mut_with_integer_key_and_value() {
    use hashbrown::hash_map::HashMap;

    let mut map: HashMap<i32, u32> = HashMap::new();
    map.insert(1, 10);

    let mut occupied_entry = match map.raw_entry_mut().from_key(&1) {
        RawEntryMut::Vacant(_) => panic!(),
        RawEntryMut::Occupied(o) => o,
    };

    let key_mut = occupied_entry.key_mut();
}

#[test]
fn test_key_mut_with_string_key_and_value() {
    use hashbrown::hash_map::HashMap;
    
    let mut map: HashMap<String, u32> = HashMap::new();
    map.insert("key".to_string(), 20);

    let mut occupied_entry = match map.raw_entry_mut().from_key(&"key".to_string()) {
        RawEntryMut::Vacant(_) => panic!(),
        RawEntryMut::Occupied(o) => o,
    };

    let key_mut = occupied_entry.key_mut();
}

#[test]
fn test_key_mut_with_tuple_key_and_value() {
    use hashbrown::hash_map::HashMap;

    let mut map: HashMap<(i32, i32), u32> = HashMap::new();
    map.insert((1, 2), 30);

    let mut occupied_entry = match map.raw_entry_mut().from_key(&(1, 2)) {
        RawEntryMut::Vacant(_) => panic!(),
        RawEntryMut::Occupied(o) => o,
    };

    let key_mut = occupied_entry.key_mut();
}

