// Answer 0

#[test]
fn test_or_insert_with_vacant_entry() {
    use hashbrown::{hash_map::RawEntryMut, HashMap, Global};

    let mut map: HashMap<&str, i32, Global> = HashMap::new();

    match map.raw_entry_mut().from_key("vacant_key") {
        RawEntryMut::Occupied(_) => unreachable!(),
        RawEntryMut::Vacant(entry) => {
            entry.or_insert_with(|| ("vacant_key", 42));
        }
    }
}

#[test]
fn test_or_insert_with_vacant_entry_with_different_key_value() {
    use hashbrown::{hash_map::RawEntryMut, HashMap, Global};

    let mut map: HashMap<&str, String, Global> = HashMap::new();

    match map.raw_entry_mut().from_key("another_vacant_key") {
        RawEntryMut::Occupied(_) => unreachable!(),
        RawEntryMut::Vacant(entry) => {
            entry.or_insert_with(|| ("another_vacant_key", "value".to_string()));
        }
    }
}

#[test]
fn test_or_insert_with_vacant_entry_multiple_calls() {
    use hashbrown::{hash_map::RawEntryMut, HashMap, Global};

    let mut map: HashMap<&str, f64, Global> = HashMap::new();

    match map.raw_entry_mut().from_key("float_key") {
        RawEntryMut::Occupied(_) => unreachable!(),
        RawEntryMut::Vacant(entry) => {
            entry.or_insert_with(|| ("float_key", 3.14));
        }
    }
    
    match map.raw_entry_mut().from_key("float_key2") {
        RawEntryMut::Occupied(_) => unreachable!(),
        RawEntryMut::Vacant(entry) => {
            entry.or_insert_with(|| ("float_key2", 2.71));
        }
    }
}

