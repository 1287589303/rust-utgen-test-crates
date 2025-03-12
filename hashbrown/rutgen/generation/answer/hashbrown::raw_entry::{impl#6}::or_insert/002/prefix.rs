// Answer 0

#[test]
fn test_or_insert_occupied_entry() {
    use hashbrown::{HashMap, raw_map::RawEntryMut};

    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("k1", 1);
    map.insert("k2", 2);

    match map.raw_entry_mut().from_key("k1") {
        RawEntryMut::Occupied(entry) => {
            let (key, value) = entry.or_insert("default_key", 42);
            // Function call doesn't need assertions as per instructions.
            let _ = (key, value);
        },
        RawEntryMut::Vacant(_) => unreachable!(),
    }
}

#[test]
fn test_or_insert_vacant_entry() {
    use hashbrown::{HashMap, raw_map::RawEntryMut};

    let mut map: HashMap<&str, u32> = HashMap::new();

    match map.raw_entry_mut().from_key("k3") {
        RawEntryMut::Occupied(_) => unreachable!(),
        RawEntryMut::Vacant(entry) => {
            let (key, value) = entry.or_insert("k3", 3);
            // Function call doesn't need assertions as per instructions.
            let _ = (key, value);
        },
    }
}

#[test]
fn test_or_insert_with_key_already_present() {
    use hashbrown::{HashMap, raw_map::RawEntryMut};

    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("existing_key", 10);

    match map.raw_entry_mut().from_key("existing_key") {
        RawEntryMut::Occupied(entry) => {
            let (key, value) = entry.or_insert("not_used", 20);
            // Function call doesn't need assertions as per instructions.
            let _ = (key, value);
        },
        RawEntryMut::Vacant(_) => unreachable!(),
    }
}

