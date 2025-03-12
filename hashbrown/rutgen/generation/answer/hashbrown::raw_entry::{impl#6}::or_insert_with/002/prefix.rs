// Answer 0

#[test]
fn test_or_insert_with_occupied_entry() {
    use hashbrown::{HashMap, hash_map::RawEntryMut};
    use std::hash::{BuildHasherDefault, Hash};
    
    // Define a simple struct and implement Hash for it
    #[derive(Hash, PartialEq, Eq)]
    struct Key {
        id: i32,
    }
    
    // Provide a default hasher
    type DefaultHasher = BuildHasherDefault<core::hash::SipHasher>;

    let mut map: HashMap<Key, String, DefaultHasher> = HashMap::new();
    
    // Insert an initial key-value pair
    map.insert(Key { id: 1 }, "occupied".to_string());
    
    // Obtain a RawEntryMut and test or_insert_with on an occupied entry
    let entry = map.raw_entry_mut().from_key(&Key { id: 1 });
    match entry {
        RawEntryMut::Occupied(entry) => {
            entry.or_insert_with(|| (Key { id: 2 }, "new_value".to_string()));
        },
        RawEntryMut::Vacant(_) => unreachable!(),
    }
}

#[test]
fn test_or_insert_with_empty_entry_default_function() {
    use hashbrown::{HashMap, hash_map::RawEntryMut};
    use std::hash::{BuildHasherDefault, Hash};
    
    #[derive(Hash, PartialEq, Eq)]
    struct Key {
        id: i32,
    }

    type DefaultHasher = BuildHasherDefault<core::hash::SipHasher>;

    let mut map: HashMap<Key, String, DefaultHasher> = HashMap::new();
    
    // Ensure there is a key-value pair so we can access an occupied entry
    map.insert(Key { id: 1 }, "occupied".to_string());
    
    // Access an existing occupied entry
    let entry = map.raw_entry_mut().from_key(&Key { id: 1 });
    match entry {
        RawEntryMut::Occupied(entry) => {
            entry.or_insert_with(|| (Key { id: 2 }, "default_value".to_string()));
        },
        RawEntryMut::Vacant(_) => unreachable!(),
    }
}

