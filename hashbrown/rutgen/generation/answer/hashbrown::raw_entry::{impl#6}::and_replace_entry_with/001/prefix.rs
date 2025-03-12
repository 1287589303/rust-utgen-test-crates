// Answer 0

#[test]
fn test_and_replace_entry_with_vacant_1() {
    use hashbrown::{HashMap, hash_map::RawEntryMut};

    let mut map: HashMap<&str, u32> = HashMap::new();

    let entry = map
        .raw_entry_mut()
        .from_key("not_here")
        .and_replace_entry_with(|_k, _v| Some(42)); // should remain vacant

    match entry {
        RawEntryMut::Vacant(_) => {},
        RawEntryMut::Occupied(_) => panic!(),
    }
}

#[test]
fn test_and_replace_entry_with_vacant_2() {
    use hashbrown::{HashMap, hash_map::RawEntryMut};

    let mut map: HashMap<&str, u32> = HashMap::new();

    map.insert("existing_key", 100);

    let entry = map
        .raw_entry_mut()
        .from_key("non_existing_key")
        .and_replace_entry_with(|_k, _v| None); // should remain vacant

    match entry {
        RawEntryMut::Vacant(_) => {},
        RawEntryMut::Occupied(_) => panic!(),
    }
}

#[test]
fn test_and_replace_entry_with_vacant_3() {
    use hashbrown::{HashMap, hash_map::RawEntryMut};

    let mut map: HashMap<&str, u32> = HashMap::new();

    let entry = map
        .raw_entry_mut()
        .from_key("missing_key")
        .and_replace_entry_with(|_k, _v| Some(999)); // should remain vacant

    match entry {
        RawEntryMut::Vacant(_) => {},
        RawEntryMut::Occupied(_) => panic!(),
    }
}

