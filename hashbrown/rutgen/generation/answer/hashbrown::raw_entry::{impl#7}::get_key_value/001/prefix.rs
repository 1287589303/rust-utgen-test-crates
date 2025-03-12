// Answer 0

#[test]
fn test_get_key_value_existing_entry() {
    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("a", 100);
    map.insert("b", 200);
    
    match map.raw_entry_mut().from_key(&"a") {
        RawEntryMut::Occupied(o) => {
            let key_value = o.get_key_value();
            // function call
            let _ = key_value;
        },
        RawEntryMut::Vacant(_) => panic!(),
    }
}

#[test]
fn test_get_key_value_another_existing_entry() {
    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("x", 300);
    map.insert("y", 400);
    
    match map.raw_entry_mut().from_key(&"x") {
        RawEntryMut::Occupied(o) => {
            let key_value = o.get_key_value();
            // function call
            let _ = key_value;
        },
        RawEntryMut::Vacant(_) => panic!(),
    }
}

#[test]
fn test_get_key_value_boundary_test() {
    let mut map: HashMap<u32, String> = HashMap::new();
    map.insert(0, "zero".to_string());
    map.insert(1, "one".to_string());

    match map.raw_entry_mut().from_key(&0) {
        RawEntryMut::Occupied(o) => {
            let key_value = o.get_key_value();
            // function call
            let _ = key_value;
        },
        RawEntryMut::Vacant(_) => panic!(),
    }
}

#[test]
fn test_get_key_value_another_boundary_test() {
    let mut map: HashMap<u32, String> = HashMap::new();
    map.insert(u32::MAX, "max".to_string());

    match map.raw_entry_mut().from_key(&u32::MAX) {
        RawEntryMut::Occupied(o) => {
            let key_value = o.get_key_value();
            // function call
            let _ = key_value;
        },
        RawEntryMut::Vacant(_) => panic!(),
    }
}

