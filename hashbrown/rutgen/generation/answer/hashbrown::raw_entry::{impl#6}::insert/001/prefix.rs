// Answer 0

#[test]
fn test_insert_vacant_entry() {
    use hashbrown::{hash_map::RawEntryMut, HashMap};
    use std::hash::BuildHasherDefault;

    struct SimpleHasher;
    impl Default for SimpleHasher {
        fn default() -> Self {
            SimpleHasher
        }
    }

    let mut map: HashMap<&str, u32, BuildHasherDefault<SimpleHasher>> = HashMap::new();
    let entry = map.raw_entry_mut().from_key("key1");
    let occupied_entry = entry.insert("key1", 100);
}

#[test]
fn test_insert_multiple_vacant_entries() {
    use hashbrown::{hash_map::RawEntryMut, HashMap};
    use std::hash::BuildHasherDefault;

    struct SimpleHasher;
    impl Default for SimpleHasher {
        fn default() -> Self {
            SimpleHasher
        }
    }

    let mut map: HashMap<&str, u32, BuildHasherDefault<SimpleHasher>> = HashMap::new();
    let entry1 = map.raw_entry_mut().from_key("key2");
    let occupied_entry1 = entry1.insert("key2", 200);

    let entry2 = map.raw_entry_mut().from_key("key3");
    let occupied_entry2 = entry2.insert("key3", 300);
}

#[test]
fn test_insert_with_different_keys() {
    use hashbrown::{hash_map::RawEntryMut, HashMap};
    use std::hash::BuildHasherDefault;

    struct SimpleHasher;
    impl Default for SimpleHasher {
        fn default() -> Self {
            SimpleHasher
        }
    }

    let mut map: HashMap<&str, u32, BuildHasherDefault<SimpleHasher>> = HashMap::new();
    let entry = map.raw_entry_mut().from_key("key4");
    let occupied_entry = entry.insert("key4", 400);
}

#[test]
fn test_insert_empty_hashmap() {
    use hashbrown::{hash_map::RawEntryMut, HashMap};
    use std::hash::BuildHasherDefault;

    struct SimpleHasher;
    impl Default for SimpleHasher {
        fn default() -> Self {
            SimpleHasher
        }
    }

    let mut map: HashMap<&str, u32, BuildHasherDefault<SimpleHasher>> = HashMap::new();
    let entry = map.raw_entry_mut().from_key("key5");
    let occupied_entry = entry.insert("key5", 500);
}

