// Answer 0

#[test]
fn test_insert_existing_value() {
    use hashbrown::hash_map::{HashMap, RawEntryMut};

    let mut map: HashMap<&str, u32> = [("a", 100), ("b", 200)].into();

    match map.raw_entry_mut().from_key(&"a") {
        RawEntryMut::Vacant(_) => panic!(),
        RawEntryMut::Occupied(mut o) => {
            let old_value = o.insert(150);
        }
    }
}

#[test]
fn test_insert_boundary_value() {
    use hashbrown::hash_map::{HashMap, RawEntryMut};

    let mut map: HashMap<&str, u32> = [("a", 100), ("b", 200)].into();

    match map.raw_entry_mut().from_key(&"b") {
        RawEntryMut::Vacant(_) => panic!(),
        RawEntryMut::Occupied(mut o) => {
            let old_value = o.insert(0);
        }
    }

    match map.raw_entry_mut().from_key(&"b") {
        RawEntryMut::Vacant(_) => panic!(),
        RawEntryMut::Occupied(mut o) => {
            let old_value = o.insert(u32::MAX);
        }
    }
}

#[test]
#[should_panic]
fn test_insert_non_existing_key() {
    use hashbrown::hash_map::{HashMap, RawEntryMut};

    let mut map: HashMap<&str, u32> = [("a", 100), ("b", 200)].into();

    match map.raw_entry_mut().from_key(&"c") {
        RawEntryMut::Vacant(_) => {
            // Simulate panic; trying to insert on a non-existing entry
            panic!();
        },
        RawEntryMut::Occupied(_) => {}
    }
}

